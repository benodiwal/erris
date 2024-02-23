use tokio::time::sleep;
use std::time::Duration;

use crate::Result;
use crate::ais::msg::{get_text_content, user_msg};
use crate::ais::{OaClient, POLLING_DURATION_MS};
use async_openai::types::{AssistantObject, AssistantToolsRetrieval, CreateAssistantRequest, CreateRunRequest, CreateThreadRequest, ModifyAssistantRequest, RunStatus, ThreadObject};
use console::Term;
use derive_more::{From, Deref, Display};

// region: -- Constants

const DEFAULT_QUERY: &[(&str, &str)] = &[("limit", "100")];

// endregion: --Constants

// region:  --Types

pub struct CreateConfig {
    pub name: String,
    pub model: String,
}

#[derive(Debug, From, Deref, Display)]
pub struct AsstId(String);

#[derive(Debug, From, Deref, Display)]
pub struct ThreadId(String);

#[derive(Debug, From, Deref, Display)]
pub struct FileId(String);

// endregion: -- Types

// region: -- Asst CRUD

pub async fn create(oac: &OaClient, config: CreateConfig) -> Result<AsstId> {

    let oa_assts = oac.assistants();
    let asst_obj = oa_assts.create(CreateAssistantRequest {
        model: config.model,
        name: Some(config.name),
        tools: Some(vec![AssistantToolsRetrieval::default().into()]),
        ..Default::default()
    })
    .await?;

    Ok(asst_obj.id.into())

}

pub async fn load_or_create_asst(
    oac: &OaClient,
    config: CreateConfig,
    recreate: bool,
) -> Result<AsstId> {
    let asst_obj = first_by_name(oac, &config.name).await?;
    let mut asst_id = asst_obj.map(|o| AsstId::from(o.id));

    if let (true, Some(asst_id_ref)) = (recreate, asst_id.as_ref()) {
        delete(oac, asst_id_ref).await?;
        asst_id.take();
        println!("Assistant {} deleted", config.name);
    }

    if let Some(asst_id) = asst_id {
        println!("Assistant {} loaded", config.name);
        Ok(asst_id)
    } else {
        // let asst_name = config.name.clone();
        let asst_id = create(oac, config).await?;
        Ok(asst_id)
    }

}

pub async fn first_by_name(
    oac: &OaClient,
    name: &str,
) -> Result<Option<AssistantObject>> {
    let oa_assts = oac.assistants();
    let assts = oa_assts.list(DEFAULT_QUERY).await?.data;
    let asst_obj = assts.into_iter().find(|a| a.name.as_ref().map(|n| n == name).unwrap_or(false));
    
    Ok(asst_obj)
}

pub async fn upload_instructions(
    oac: &OaClient,
    asst_id: &AsstId,
    inst_content: String,
) -> Result<()> {
    let oa_assts = oac.assistants();
    let modif = ModifyAssistantRequest {
        instructions: Some(inst_content),
        ..Default::default()
    };
    oa_assts.update(asst_id, modif).await?;

    Ok(())
}

pub async fn delete(oac: &OaClient, asst_id: &AsstId) -> Result<()> {
    let oa_assts = oac.assistants();
    oa_assts.delete(asst_id).await?;
    Ok(())
}

// endregion: -- Asst CRUD

// region : -- Thread

pub async fn create_thread(
    oac: &OaClient
) -> Result<ThreadId> {
    
    let oa_threads = oac.threads();
    
    let res = oa_threads.create(CreateThreadRequest {
        ..Default::default()
    })
    .await?;
    
    Ok(res.id.into())               
}

pub async fn get_thread(
    oac: &OaClient,
    thread_id: &ThreadId,
) -> Result<ThreadObject> {

    let oa_threads = oac.threads();
    let thread_obj = oa_threads.retrieve(thread_id).await?;
    Ok(thread_obj)
}

pub async fn run_thread_msg(
    oac: &OaClient,
    asst_id: &AsstId,
    thread_id: &ThreadId,
    msg: &str,
) -> Result<String> {

    let msg = user_msg(msg);

    let _message_obj = oac.threads().messages(thread_id).create(msg).await?;

    let run_request = CreateRunRequest {
        assistant_id: asst_id.to_string(),
        ..Default::default()
    };

    let run = oac.threads().runs(thread_id).create(run_request).await?;

    let term = Term::stdout();
  
    loop {
        term.write_str(">")?;
        let run  = oac.threads().runs(thread_id).retrieve(&run.id).await?;
        term.write_str("< ")?;

        match run.status {
            RunStatus::Completed => {
                term.write_str("\n")?;                
                return get_first_thread_msg_content(oac, thread_id).await;
            }
            RunStatus::Queued | RunStatus::InProgress => (),
            other => {
                term.write_str("\n")?;
                return Err(format!("ERROR WHILE RUN: {:?}", other).into());
            }
        }

        sleep(Duration::from_millis(POLLING_DURATION_MS)).await;
    }

}

pub async fn get_first_thread_msg_content(oac: &OaClient, thread_id: &ThreadId) -> Result<String> {

    static QUERY: [(&str, &str); 1] = [("limit", "1")];
    let messages = oac.threads().messages(thread_id).list(&QUERY).await?;
    let msg = messages.data.into_iter().next().ok_or_else(|| "No message found".to_string())?;

    let text = get_text_content(msg)?;    
    Ok(text)
    
}

// endregion : --Thread