# Pinyin UI

This project provides a user interface for voice listening and converting spoken input into Pinyin English.

## Features

- Captures audio input from the microphone.
- Uses Voice Activity Detection (VAD) to identify speech segments.
- Transcribes spoken Chinese audio using the Whisper model.
- Translates the transcribed Chinese text into Pinyin English using a Llama language model with a specific system prompt.

## Technologies Used

- [Kalosm](https://kalosm.ai/): A Rust library for AI and machine learning, used here for audio processing and language model integration.
- Whisper: An open-source speech recognition model.
- Llama: A large language model used for translation.

## Setup and Running

*(Note: Specific setup instructions may vary depending on your environment and how Kalosm is configured. These are general steps.)*

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd pinyin-ui
    ```
2.  **Ensure Rust and Cargo are installed:**
    If not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
3.  **Build and run the project (optimized):**
    ```bash
    cargo run --release
    ```
    Alternatively, you can build the release version without running:
    ```bash
    cargo build --release
    ```

The application will load the necessary models and start listening for microphone input. Transcribed and translated text will be printed to the console.
