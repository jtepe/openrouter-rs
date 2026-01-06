use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// OpenResponsesRequest
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    /// OpenResponsesInput
    pub input: Option<Input>,
    /// instructions
    pub instructions: Option<String>,
    /// OpenResponsesRequestMetadata
    pub metadata: Option<HashMap<String, String>>,
    /// OpenResponsesRequestToolsItems
    pub tools: Option<Vec<Tool>>,
    /// OpenAIResponsesToolChoice
    pub tool_choice: Option<ToolChoice>,
    /// parallel_tool_calls
    pub parallel_tool_calls: Option<bool>,
    /// model
    pub model: Option<String>,
    /// models
    pub models: Option<Vec<String>>,
    /// OpenResponsesResponseText
    pub text: Option<ResponseTextConfig>,
    /// OpenResponsesReasoningConfig
    pub reasoning: Option<ReasoningConfig>,
    /// max_output_tokens
    pub max_output_tokens: Option<u32>,
    /// temperature
    pub temperature: Option<f32>,
    /// top_p
    pub top_p: Option<f32>,
    /// top_k
    pub top_k: Option<u32>,
    /// prompt_cache_key
    pub prompt_cache_key: Option<String>,
    /// previous_response_id
    pub previous_response_id: Option<String>,
    /// OpenAIResponsesPrompt
    pub prompt: Option<Prompt>,
    /// OpenAIResponsesIncludable
    pub include: Option<Vec<Includable>>,
    /// background
    pub background: Option<bool>,
    /// safety_identifier
    pub safety_identifier: Option<String>,
    /// store
    pub store: Option<bool>,
    /// OpenResponsesRequestServiceTier
    pub service_tier: Option<ServiceTier>,
    /// OpenResponsesRequestTruncation
    pub truncation: Option<Truncation>,
    /// stream
    #[serde(default)]
    pub stream: bool,
    /// OpenResponsesRequestProvider
    pub provider: Option<Provider>,
    /// OpenResponsesRequestPluginsItems
    pub plugins: Option<Vec<Plugin>>,
    /// user
    pub user: Option<String>,
    /// session_id
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
    InputMessage(InputMessageItem),
    EasyInputMessage(EasyInputMessage),
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reasoning {
    /// OutputItemReasoningType
    #[serde(rename = "type")]
    pub type_: String, // "reasoning"
    pub id: String,
    pub content: Vec<ReasoningTextContent>,
    pub summary: Vec<ReasoningSummaryText>,
    pub encrypted_content: Option<String>,
    pub status: Option<ReasoningStatus>, // "completed" | "incomplete" | "in_progress"
    pub signature: Option<String>,
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
#[skip_serializing_none]
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
        file_data: Option<String>,
        filename: Option<String>,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputMessageItem {
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionCallOutput {
    /// OpenResponsesFunctionCallOutputType
    #[serde(rename = "type")]
    pub type_: String, // "function_call_output"
    pub id: Option<String>,
    pub call_id: String,
    pub output: String,
    pub status: Option<ToolCallStatus>,
}

/// ResponsesOutputMessage
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputMessage {
    pub id: String,
    /// OutputMessageRole
    pub role: String, // "assistant"
    /// OutputMessageType
    #[serde(rename = "type")]
    pub type_: String, // "message"
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
        index: u32,
    },
    UrlCitation {
        url: String,
        title: String,
        start_index: u32,
        end_index: u32,
    },
    FilePath {
        file_id: String,
        index: u32,
    },
}

/// ResponsesOutputItemReasoning
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputItemReasoning {
    /// OutputItemReasoningType
    #[serde(rename = "type")]
    pub type_: String, // "reasoning"
    pub id: String,
    pub content: Vec<ReasoningTextContent>,
    pub summary: Vec<ReasoningSummaryText>,
    pub encrypted_content: Option<String>,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputItemFunctionCall {
    /// OutputItemFunctionCallType
    #[serde(rename = "type")]
    pub type_: String, // "function_call"
    pub id: String,
    pub name: String,
    pub arguments: String,
    pub call_id: String,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageGenerationCall {
    /// OutputItemImageGenerationCallType
    #[serde(rename = "type")]
    pub type_: String, // "image_generation_call"
    pub id: String,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Tool {
    Function {
        name: String,
        description: Option<String>,
        strict: Option<bool>,
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
        filters: Option<WebSearchToolFilters>,
        search_context_size: SearchContextSize,
        user_location: WebSearchUserLocation,
    },
    #[serde(rename = "web_search_2025_08_26")]
    WebSearch20250826 {
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchPreviewToolUserLocation {
    /// WebSearchPreviewToolUserLocationType
    #[serde(rename = "type")]
    pub type_: String, // "approximate"
    pub city: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
}

/// ResponsesWebSearchUserLocation
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchUserLocation {
    /// ResponsesWebSearchUserLocationType
    #[serde(rename = "type")]
    pub type_: String, // "approximate"
    pub city: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
}

/// OpenResponsesWebSearchToolFilters
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchToolFilters {
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseTextConfig {
    pub format: ResponseFormat,
    pub verbosity: Option<ResponseTextVerbosity>,
}

/// ResponseFormatTextConfig
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
    JsonSchema {
        name: String,
        description: String,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReasoningConfig {
    pub effort: ReasoningEffort,
    pub summary: ReasoningSummaryVerbosity,
    pub max_tokens: Option<u32>,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    pub id: String,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Provider {
    pub allow_fallbacks: Option<bool>,
    pub require_parameters: Option<bool>,
    pub data_collection: Option<DataCollection>,
    pub zdr: Option<bool>,
    pub enforce_distillable_text: Option<bool>,
    pub order: Option<Vec<String>>, // Using String for slugs as ProviderName enum is huge and might change
    pub only: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>,
    pub quantizations: Option<Vec<Quantization>>,
    pub sort: Option<ProviderSort>,
    pub max_price: Option<ProviderMaxPrice>,
    pub preferred_min_throughput: Option<PreferredMinThroughput>,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderSortConfig {
    pub by: Option<ProviderSortStrategy>,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderMaxPrice {
    pub prompt: Option<f64>,
    pub completion: Option<f64>,
    pub image: Option<f64>,
    pub audio: Option<f64>,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PercentileThroughputCutoffs {
    pub p50: Option<f64>,
    pub p75: Option<f64>,
    pub p90: Option<f64>,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PercentileLatencyCutoffs {
    pub p50: Option<f64>,
    pub p75: Option<f64>,
    pub p90: Option<f64>,
    pub p99: Option<f64>,
}

/// OpenResponsesRequestPluginsItems
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "id")]
#[serde(rename_all = "kebab-case")]
pub enum Plugin {
    AutoRouter {
        enabled: Option<bool>,
        allowed_models: Option<Vec<String>>,
    },
    Moderation,
    Web {
        enabled: Option<bool>,
        max_results: Option<u32>,
        search_prompt: Option<String>,
        engine: Option<WebSearchEngine>,
    },
    FileParser {
        enabled: Option<bool>,
        pdf: Option<PdfParserOptions>,
    },
    ResponseHealing {
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PdfParserOptions {
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub id: String,
    /// OpenAiResponsesNonStreamingResponseObject
    pub object: String, // "response"
    pub created_at: u64,
    pub model: String,
    /// OpenAIResponsesResponseStatus
    pub status: ResponseStatus,
    pub output: Vec<OutputItem>,
    pub user: Option<String>,
    pub output_text: Option<String>,
    pub prompt_cache_key: Option<String>,
    pub safety_identifier: Option<String>,
    pub error: Option<ErrorField>,
    pub incomplete_details: Option<IncompleteDetails>,
    pub usage: Option<Usage>,
    pub max_tool_calls: Option<u32>,
    pub top_logprobs: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub instructions: Option<Input>,
    pub metadata: Option<HashMap<String, String>>,
    pub tools: Option<Vec<Tool>>,
    pub tool_choice: Option<ToolChoice>,
    pub parallel_tool_calls: Option<bool>,
    pub prompt: Option<Prompt>,
    pub background: Option<bool>,
    pub previous_response_id: Option<String>,
    pub reasoning: Option<ReasoningConfig>,
    pub service_tier: Option<ServiceTier>,
    pub store: Option<bool>,
    pub truncation: Option<TruncationEnum>,
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
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub input_tokens: u32,
    pub input_tokens_details: InputTokensDetails,
    pub output_tokens: u32,
    pub output_tokens_details: OutputTokensDetails,
    pub total_tokens: u32,
    pub cost: Option<f64>,
    pub is_byok: Option<bool>,
    pub cost_details: Option<CostDetails>,
}

/// OpenAiResponsesUsageInputTokensDetails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputTokensDetails {
    pub cached_tokens: u32,
}

/// OpenAiResponsesUsageOutputTokensDetails
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputTokensDetails {
    pub reasoning_tokens: u32,
}

/// OpenResponsesUsageCostDetails
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CostDetails {
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
