use ferrisoxide_core::csv::{CsvParseOptions, SimpleCsvParser, WaveformParser};

#[test]
fn parses_basic_waveform_fixture() {
    let input = include_str!("../../../tests/fixtures/basic_waveform.csv");
    let parser = SimpleCsvParser;
    let options = CsvParseOptions::new("time", vec!["input_v".to_string(), "output_v".to_string()]);

    let waveform = parser
        .parse_str(input, &options)
        .expect("fixture should parse");

    assert_eq!(waveform.sample_count(), 5);
    assert_eq!(waveform.channels.len(), 2);
    assert_eq!(waveform.channels[0].name, "input_v");
    assert_eq!(waveform.channels[1].samples[4], 4.9);
}
