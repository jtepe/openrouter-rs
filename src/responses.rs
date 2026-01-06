use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OpenResponsesRequest
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    /// OpenResponsesInput
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
    /// instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// OpenResponsesRequestMetadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// OpenResponsesRequestToolsItems
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// OpenAIResponsesToolChoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// parallel_tool_calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    /// OpenResponsesResponseText
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,
    /// OpenResponsesReasoningConfig
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    /// max_output_tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,
    /// temperature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// top_p
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// top_k
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    /// prompt_cache_key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// previous_response_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// OpenAIResponsesPrompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,
    /// OpenAIResponsesIncludable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<Includable>>,
    /// background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// safety_identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    /// store
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// OpenResponsesRequestServiceTier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    /// OpenResponsesRequestTruncation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,
    /// stream
    #[serde(default)]
    pub stream: bool,
    /// OpenResponsesRequestProvider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    /// OpenResponsesRequestPluginsItems
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<Plugin>>,
    /// user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// session_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

/// OpenResponsesInput
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Input {
    String(String),
    List(Vec<InputItem>),
}

/// OpenResponsesInputOneOf1Items
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputItem {
    Reasoning(Reasoning),
    EasyInputMessage(EasyInputMessage),
    InputMessage(InputMessageItem),
    FunctionToolCall(FunctionToolCall),
    FunctionCallOutput(FunctionCallOutput),
    OutputMessage(OutputMessage),
    OutputItemReasoning(OutputItemReasoning),
    OutputItemFunctionCall(OutputItemFunctionCall),
    WebSearchCallOutput(WebSearchCallOutput),
    OutputItemFileSearchCall(OutputItemFileSearchCall),
    ImageGenerationCall(ImageGenerationCall),
}

/// OpenResponsesReasoning
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reasoning {
    /// OutputItemReasoningType
    #[serde(rename = "type")]
    pub type_: String, // "reasoning"
    pub id: String,
    pub content: Vec<ReasoningTextContent>,
    pub summary: Vec<ReasoningSummaryText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReasoningStatus>, // "completed" | "incomplete" | "in_progress"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ReasoningFormat>,
}

/// OpenResponsesEasyInputMessage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EasyInputMessage {
    /// OpenResponsesEasyInputMessageType
    #[serde(rename = "type")]
    pub type_: String, // "message"
    /// OpenResponsesEasyInputMessageRole
    pub role: EasyInputMessageRole,
    /// OpenResponsesEasyInputMessageContent
    pub content: EasyInputMessageContent,
}

/// OpenResponsesEasyInputMessageRole
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EasyInputMessageRole {
    User,
    System,
    Assistant,
    Developer,
}

/// OpenResponsesEasyInputMessageContent
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EasyInputMessageContent {
    String(String),
    Parts(Vec<EasyInputMessageContentPart>),
}

/// OpenResponsesEasyInputMessageContentOneOf0Items
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EasyInputMessageContentPart {
    InputText { text: String },
    InputImage {
        detail: ImageDetail,
        image_url: Option<String>,
    },
    InputFile {
        file_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        file_url: Option<String>,
    },
    InputAudio {
        input_audio: AudioInput,
    },
}

/// ResponseInputImageDetail
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Auto,
    High,
    Low,
}

/// ResponseInputAudioInputAudio
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioInput {
    pub data: String,
    pub format: AudioFormat,
}

/// ResponseInputAudioInputAudioFormat
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    Mp3,
    Wav,
}

/// OpenResponsesInputMessageItem
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputMessageItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// OpenResponsesInputMessageItemType
    #[serde(rename = "type")]
    pub type_: String, // "message"
    /// OpenResponsesInputMessageItemRole
    pub role: InputMessageItemRole,
    /// OpenResponsesInputMessageItemContentItems
    pub content: Vec<EasyInputMessageContentPart>,
}

/// OpenResponsesInputMessageItemRole
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum InputMessageItemRole {
    User,
    System,
    Developer,
}

/// OpenResponsesFunctionToolCall
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionToolCall {
    /// OpenResponsesFunctionToolCallType
    #[serde(rename = "type")]
    pub type_: String, // "function_call"
    pub call_id: String,
    pub name: String,
    pub arguments: String,
    pub id: String,
    pub status: ToolCallStatus,
}

/// ToolCallStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallStatus {
    InProgress,
    Completed,
    Incomplete,
}

/// OpenResponsesFunctionCallOutput
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCallOutput {
    /// OpenResponsesFunctionCallOutputType
    #[serde(rename = "type")]
    pub type_: String, // "function_call_output"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub call_id: String,
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ToolCallStatus>,
}

/// ResponsesOutputMessage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputMessage {
    pub id: String,
    /// OutputMessageRole
    pub role: String, // "assistant"
    /// OutputMessageType
    #[serde(rename = "type")]
    pub type_: String, // "message"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OutputMessageStatus>,
    pub content: Vec<OutputMessageContent>,
}

/// OutputMessageStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OutputMessageStatus {
    Completed,
    Incomplete,
    InProgress,
}

/// OutputMessageContentItems
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum OutputMessageContent {
    OutputText {
        text: String,
        #[serde(default)]
        annotations: Vec<Annotation>,
    },
    Refusal {
        refusal: String,
    },
}

/// OpenAIResponsesAnnotation
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Annotation {
    FileCitation {
        file_id: String,
        filename: String,
        index: f64,
    },
    UrlCitation {
        url: String,
        title: String,
        start_index: f64,
        end_index: f64,
    },
    FilePath {
        file_id: String,
        index: f64,
    },
}

/// ResponsesOutputItemReasoning
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputItemReasoning {
    /// OutputItemReasoningType
    #[serde(rename = "type")]
    pub type_: String, // "reasoning"
    pub id: String,
    pub content: Vec<ReasoningTextContent>,
    pub summary: Vec<ReasoningSummaryText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReasoningStatus>,
}

/// ReasoningTextContent
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReasoningTextContent {
    /// ReasoningTextContentType
    #[serde(rename = "type")]
    pub type_: String, // "reasoning_text"
    pub text: String,
}

/// ReasoningSummaryText
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReasoningSummaryText {
    /// ReasoningSummaryTextType
    #[serde(rename = "type")]
    pub type_: String, // "summary_text"
    pub text: String,
}

/// OutputItemReasoningStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningStatus {
    Completed,
    Incomplete,
    InProgress,
}

/// OpenResponsesReasoningFormat
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReasoningFormat {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "openai-responses-v1")]
    OpenAIResponsesV1,
    #[serde(rename = "azure-openai-responses-v1")]
    AzureOpenAIResponsesV1,
    #[serde(rename = "xai-responses-v1")]
    XAIResponsesV1,
    #[serde(rename = "anthropic-claude-v1")]
    AnthropicClaudeV1,
    #[serde(rename = "google-gemini-v1")]
    GoogleGeminiV1,
}

/// ResponsesOutputItemFunctionCall
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputItemFunctionCall {
    /// OutputItemFunctionCallType
    #[serde(rename = "type")]
    pub type_: String, // "function_call"
    pub id: String,
    pub name: String,
    pub arguments: String,
    pub call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionCallStatus>,
}

/// OutputItemFunctionCallStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FunctionCallStatus {
    Completed,
    Incomplete,
    InProgress,
}

/// ResponsesWebSearchCallOutput
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchCallOutput {
    /// OutputItemWebSearchCallType
    #[serde(rename = "type")]
    pub type_: String, // "web_search_call"
    pub id: String,
    pub status: WebSearchStatus,
}

/// WebSearchStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WebSearchStatus {
    Completed,
    Searching,
    InProgress,
    Failed,
}

/// ResponsesOutputItemFileSearchCall
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputItemFileSearchCall {
    /// OutputItemFileSearchCallType
    #[serde(rename = "type")]
    pub type_: String, // "file_search_call"
    pub id: String,
    pub queries: Vec<String>,
    pub status: WebSearchStatus,
}

/// ResponsesImageGenerationCall
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageGenerationCall {
    /// OutputItemImageGenerationCallType
    #[serde(rename = "type")]
    pub type_: String, // "image_generation_call"
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    pub status: ImageGenerationStatus,
}

/// ImageGenerationStatus
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenerationStatus {
    InProgress,
    Completed,
    Generating,
    Failed,
}

/// OpenResponsesRequestToolsItems
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Tool {
    Function {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<serde_json::Value>,
    },
    WebSearchPreview {
        search_context_size: SearchContextSize,
        user_location: WebSearchPreviewToolUserLocation,
    },
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311 {
        search_context_size: SearchContextSize,
        user_location: WebSearchPreviewToolUserLocation,
    },
    WebSearch {
        #[serde(skip_serializing_if = "Option::is_none")]
        filters: Option<WebSearchToolFilters>,
        search_context_size: SearchContextSize,
        user_location: WebSearchUserLocation,
    },
    #[serde(rename = "web_search_2025_08_26")]
    WebSearch20250826 {
        #[serde(skip_serializing_if = "Option::is_none")]
        filters: Option<WebSearchToolFilters>,
        search_context_size: SearchContextSize,
        user_location: WebSearchUserLocation,
    },
}

/// ResponsesSearchContextSize
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SearchContextSize {
    Low,
    Medium,
    High,
}

/// WebSearchPreviewToolUserLocation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchPreviewToolUserLocation {
    /// WebSearchPreviewToolUserLocationType
    #[serde(rename = "type")]
    pub type_: String, // "approximate"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// ResponsesWebSearchUserLocation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchUserLocation {
    /// ResponsesWebSearchUserLocationType
    #[serde(rename = "type")]
    pub type_: String, // "approximate"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// OpenResponsesWebSearchToolFilters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchToolFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
}

/// OpenAIResponsesToolChoice
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    Mode(ToolChoiceMode),
    Tool {
        #[serde(rename = "type")]
        type_: ToolChoiceType,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceMode {
    Auto,
    None,
    Required,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceType {
    Function,
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311,
    WebSearchPreview,
}

/// OpenResponsesResponseText
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTextConfig {
    pub format: ResponseFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<ResponseTextVerbosity>,
}

/// ResponseFormatTextConfig
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema {
        name: String,
        description: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
        schema: serde_json::Value,
    },
}

/// ResponseTextConfigVerbosity
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseTextVerbosity {
    High,
    Low,
    Medium,
}

/// OpenResponsesReasoningConfig
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReasoningConfig {
    pub effort: ReasoningEffort,
    pub summary: ReasoningSummaryVerbosity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// OpenAIResponsesReasoningEffort
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    Xhigh,
    High,
    Medium,
    Low,
    Minimal,
    None,
}

/// ReasoningSummaryVerbosity
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningSummaryVerbosity {
    Auto,
    Concise,
    Detailed,
}

/// OpenAIResponsesPrompt
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, PromptVariable>>,
}

/// OpenAiResponsesPromptVariables
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PromptVariable {
    String(String),
    Input(EasyInputMessageContentPart),
}

/// OpenAIResponsesIncludable
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Includable {
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageImageUrl,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrl,
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
    #[serde(rename = "code_interpreter_call.outputs")]
    CodeInterpreterCallOutputs,
}

/// OpenResponsesRequestServiceTier
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
    Priority,
    Scale,
}

/// OpenResponsesRequestTruncation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Truncation {}

/// OpenResponsesRequestProvider
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Provider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_fallbacks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_parameters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<DataCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zdr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_distillable_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<String>>, // Using String for slugs as ProviderName enum is huge and might change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantizations: Option<Vec<Quantization>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ProviderSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<ProviderMaxPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_min_throughput: Option<PreferredMinThroughput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_max_latency: Option<PreferredMaxLatency>,
}

/// DataCollection
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DataCollection {
    Deny,
    Allow,
}

/// Quantization
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Quantization {
    Int4,
    Int8,
    Fp4,
    Fp6,
    Fp8,
    Fp16,
    Bf16,
    Fp32,
    Unknown,
}

/// OpenResponsesRequestProviderSort
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ProviderSort {
    Strategy(ProviderSortStrategy),
    Config(ProviderSortConfig),
}

/// ProviderSort
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSortStrategy {
    Price,
    Throughput,
    Latency,
}

/// ProviderSortConfig
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderSortConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<ProviderSortStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<ProviderSortConfigPartition>,
}

/// ProviderSortConfigPartition
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSortConfigPartition {
    Model,
    None,
}

/// OpenResponsesRequestProviderMaxPrice
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderMaxPrice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<f64>,
}

/// PreferredMinThroughput
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PreferredMinThroughput {
    Value(f64),
    Cutoffs(PercentileThroughputCutoffs),
}

/// PercentileThroughputCutoffs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PercentileThroughputCutoffs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p50: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p75: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p90: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p99: Option<f64>,
}

/// PreferredMaxLatency
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PreferredMaxLatency {
    Value(f64),
    Cutoffs(PercentileLatencyCutoffs),
}

/// PercentileLatencyCutoffs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PercentileLatencyCutoffs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p50: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p75: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p90: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p99: Option<f64>,
}

/// OpenResponsesRequestPluginsItems
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "id")]
#[serde(rename_all = "kebab-case")]
pub enum Plugin {
    AutoRouter {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_models: Option<Vec<String>>,
    },
    Moderation,
    Web {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_results: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        search_prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        engine: Option<WebSearchEngine>,
    },
    FileParser {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pdf: Option<PdfParserOptions>,
    },
    ResponseHealing {
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled: Option<bool>,
    },
}

/// WebSearchEngine
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WebSearchEngine {
    Native,
    Exa,
}

/// PDFParserOptions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PdfParserOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<PdfParserEngine>,
}

/// PDFParserEngine
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum PdfParserEngine {
    MistralOcr,
    PdfText,
    Native,
}

/// OpenResponsesNonStreamingResponse
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub id: String,
    /// OpenAiResponsesNonStreamingResponseObject
    pub object: String, // "response"
    pub created_at: f64,
    pub model: String,
    /// OpenAIResponsesResponseStatus
    pub status: ResponseStatus,
    pub output: Vec<OutputItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Input>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,
}

/// OpenAIResponsesResponseStatus
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Completed,
    Incomplete,
    InProgress,
    Failed,
    Cancelled,
    Queued,
}

/// ResponsesOutputItem
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OutputItem {
    Message(OutputMessage),
    Reasoning(OutputItemReasoning),
    FunctionCall(OutputItemFunctionCall),
    WebSearchCall(WebSearchCallOutput),
    FileSearchCall(OutputItemFileSearchCall),
    ImageGenerationCall(ImageGenerationCall),
}

/// ResponsesErrorField
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorField {
    pub code: ErrorCode,
    pub message: String,
}

/// ResponsesErrorFieldCode
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    ServerError,
    RateLimitExceeded,
    InvalidPrompt,
    VectorStoreTimeout,
    InvalidImage,
    InvalidImageFormat,
    InvalidBase64Image,
    InvalidImageUrl,
    ImageTooLarge,
    ImageTooSmall,
    ImageParseError,
    ImageContentPolicyViolation,
    InvalidImageMode,
    ImageFileTooLarge,
    UnsupportedImageMediaType,
    EmptyImageFile,
    FailedToDownloadImage,
    ImageFileNotFound,
}

/// OpenAIResponsesIncompleteDetails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncompleteDetails {
    pub reason: IncompleteDetailsReason,
}

/// OpenAiResponsesIncompleteDetailsReason
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IncompleteDetailsReason {
    MaxOutputTokens,
    ContentFilter,
}

/// OpenResponsesUsage
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub input_tokens: f64,
    pub input_tokens_details: InputTokensDetails,
    pub output_tokens: f64,
    pub output_tokens_details: OutputTokensDetails,
    pub total_tokens: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_byok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_details: Option<CostDetails>,
}

/// OpenAiResponsesUsageInputTokensDetails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputTokensDetails {
    pub cached_tokens: f64,
}

/// OpenAiResponsesUsageOutputTokensDetails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputTokensDetails {
    pub reasoning_tokens: f64,
}

/// OpenResponsesUsageCostDetails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CostDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_inference_cost: Option<f64>,
    pub upstream_inference_input_cost: f64,
    pub upstream_inference_output_cost: f64,
}

/// OpenAIResponsesTruncation
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TruncationEnum {
    Auto,
    Disabled,
}
