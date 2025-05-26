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
    let vad_stream = mic.stream().voice_activity_stream();
    let mut audio_chunks = vad_stream
        .inspect(move |vad_output| {
            let samples_count = vad_output.samples.clone().count();
            // if samples_count > 0 {
            //     println!("samples_count : {samples_count}");
            // }
        })
        .rechunk_voice_activity()
        .with_end_window(std::time::Duration::from_millis(400))
        .with_end_threshold(0.25)
        .with_time_before_speech(std::time::Duration::from_millis(200));

    loop {
        let input_audio_chunk = match tokio::time::timeout(
            std::time::Duration::from_millis(50), // Short timeout to remain responsive to is_listening_shared
            audio_chunks.next(),
        )
        .await
        {
            Ok(Some(chunk)) => chunk,
            Ok(None) => break,  // Stream ended
            Err(_) => continue, // Timeout, loop back to check is_listening_shared
        };

        let mut current_segment_text = String::new();
        let mut transcribed_stream = whisper_model.transcribe(input_audio_chunk);

        while let Some(transcribed) = transcribed_stream.next().await {
            if transcribed.probability_of_no_speech() < 0.85 {
                current_segment_text.push_str(transcribed.text());
                println!("Transcribed: {}", current_segment_text.clone());
            }
        }
    }

    let stream = mic.stream();

    // Transcribe the audio into text in chunks based on voice activity.
    let mut text_stream = stream.transcribe(model);

    // Finally, print the text to the console
    text_stream.to_std_out().await.unwrap();

    Ok(())
}
