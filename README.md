# Erris

A quirky terminal-based assistant designed to help programmers with their coding queries and development tasks.

## Installation  
### Prerequisites  
- **Rust**: Ensure you have Rust installed on your system. You can download it from rust-lang.org.
- **OpenAI API Key**: Obtain an API key from OpenAI.

### Clone the Respository  
```
$ git clone git@github.com:benodiwal/erris.git
$ cd erris
```

### Build the Project
Use Cargo, the Rust package manager, to build the project:  
```
$ cargo build --release
```

### Running the Client
Before running the client, set your OpenAI API key as an environment variable:
```
$ export OPENAI_API_KEY="your_openai_api_key"
```
Then, you can run the client with:
```
$ cargo run --release
```
