use kalosm::language::*;
use kalosm::sound::*;

const SYSTEM_PROMPT: &str = "You are an expert translator. Translate the given Chinese text to Pinyin English accurately and concisely. Output only the English translation. Do not add any pleasantries or extra explanations.";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("############# APP START #############");
    println!("Load Whisper model ...");
    // Create a new whisper model.
    let model = Whisper::new().await?;

    let whisper_model = WhisperBuilder::default()
        .with_language(Some(WhisperLanguage::Chinese)) // Specify Japanese
        .build()
        .await?;

    println!("Whisper model loaded. Initializing Llama...");

    let llama_model = Llama::builder()
        .with_source(LlamaSource::qwen_2_5_7b_instruct()) // Or another suitable model
        .build()
        .await?;
    let llama_chat_template = llama_model.chat().with_system_prompt(SYSTEM_PROMPT);

    println!("All models loaded. Listening for microphone input...");

    // Stream audio from the microphone
    let mic = MicInput::default();
    let stream = mic.stream();

    // Transcribe the audio into text in chunks based on voice activity.
    let mut text_stream = stream.transcribe(model);

    // Finally, print the text to the console
    text_stream.to_std_out().await.unwrap();

    Ok(())
}
