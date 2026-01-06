use openrouter::responses::*;
use serde_json::json;

#[test]
fn test_request_serialization() {
    let request = Request {
        input: Some(Input::List(vec![
            InputItem::EasyInputMessage(EasyInputMessage {
                type_: "message".to_string(),
                role: EasyInputMessageRole::User,
                content: EasyInputMessageContent::String("Hello, how are you?".to_string()),
            })
        ])),
        instructions: None,
        metadata: None,
        tools: Some(vec![
            Tool::Function {
                name: "get_current_weather".to_string(),
                description: Some("Get the current weather in a given location".to_string()),
                strict: None,
                parameters: Some(json!({
                    "type": "object",
                    "properties": {
                        "location": { "type": "string" }
                    }
                })),
            }
        ]),
        tool_choice: None,
        parallel_tool_calls: None,
        model: Some("anthropic/claude-4.5-sonnet-20250929".to_string()),
        models: None,
        text: None,
        reasoning: None,
        max_output_tokens: None,
        temperature: Some(0.7),
        top_p: Some(0.9),
        top_k: None,
        prompt_cache_key: None,
        previous_response_id: None,
        prompt: None,
        include: None,
        background: None,
        safety_identifier: None,
        store: None,
        service_tier: None,
        truncation: None,
        stream: false,
        provider: None,
        plugins: None,
        user: None,
        session_id: None,
    };

    let json_output = serde_json::to_string_pretty(&request).unwrap();
    println!("Serialized Request: {}", json_output);

    let deserialized: Request = serde_json::from_str(&json_output).unwrap();
    assert_eq!(deserialized.model, Some("anthropic/claude-4.5-sonnet-20250929".to_string()));
}

#[test]
fn test_response_deserialization() {
    let json_data = r#"
    {
      "id": "gen-123",
      "object": "response",
      "created_at": 1678900000.0,
      "model": "anthropic/claude-v1",
      "status": "completed",
      "output": [
        {
          "id": "msg-123",
          "role": "assistant",
          "type": "message",
          "status": "completed",
          "content": [
            {
              "type": "output_text",
              "text": "Hello! How can I help you today?"
            }
          ]
        }
      ],
      "usage": {
        "input_tokens": 10.0,
        "input_tokens_details": { "cached_tokens": 0.0 },
        "output_tokens": 20.0,
        "output_tokens_details": { "reasoning_tokens": 0.0 },
        "total_tokens": 30.0,
        "upstream_inference_input_cost": 0.0,
        "upstream_inference_output_cost": 0.0
      },
      "instructions": "some instruction",
      "metadata": {},
      "tools": [],
      "tool_choice": "auto",
      "parallel_tool_calls": false,
      "temperature": 0.7,
      "top_p": 1.0
    }
    "#;

    let response: Response = serde_json::from_str(json_data).expect("Failed to deserialize response");
    assert_eq!(response.id, "gen-123");
    assert_eq!(response.status, ResponseStatus::Completed);
    if let Some(OutputItem::Message(msg)) = response.output.first() {
        assert_eq!(msg.content.len(), 1);
    } else {
        panic!("Expected output message");
    }
}
