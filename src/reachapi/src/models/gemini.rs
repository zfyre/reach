/// This module contains the structures and enums used for the Gemini API responses.
/// Uses the following documentation:
/// https://ai.google.dev/api/generate-content


use super::{Deserialize, Serialize};


// ============================ GEMINI API RESPONSE STRUCTURES =================== //


/// Response from the model supporting multiple candidate responses.
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateContentResponse {
    /// Candidate responses from the model.
    pub candidates: Vec<Candidate>,

    /// Returns the prompt's feedback related to the content filters.
    #[serde(rename = "promptFeedback")]
    pub prompt_feedback: Option<PromptFeedback>,

    /// Output only. Metadata on the generation requests' token usage.
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<UsageMetadata>,

    /// Output only. The model version used to generate the response.
    #[serde(rename = "modelVersion")]
    pub model_version: Option<String>,
}

/// A response candidate generated from the model.
#[derive(Debug, Deserialize, Serialize)]
pub struct Candidate {
    /// Output only. Generated content returned from the model.
    pub content: Content,
    
    /// Optional. Output only. The reason why the model stopped generating tokens. If empty, the model has not stopped generating tokens.
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<FinishReason>,
    
    /// List of ratings for the safety of a response candidate. There is at most one rating per category.
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Option<Vec<SafetyRatings>>,

    /// Output only. Citation information for model-generated candidate.
    /// This field may be populated with recitation information for any text included in the content.
    /// These are passages that are "recited" from copyrighted material in the foundational LLM's training data
    #[serde(rename = "citationMetadata")]
    pub citation_metadata: Option<CitationMetadata>,

    /// Output only. Token count for this candidate.
    #[serde(rename = "tokenCount")]
    pub token_count: Option<u32>,

    /// Output only. Attribution information for sources that contributed to a grounded answer.
    /// This field is populated for GenerateAnswer calls.
    #[serde(rename = "groundingAttributions")]
    pub grounding_attributions: Option<Vec<GroundingAttribution>>,

    /// Output only. Grounding metadata for the candidate.
    /// This field is populated for GenerateContent calls.
    #[serde(rename = "groundingMetadata")]  
    pub grounding_metadata: Option<Vec<GroundingMetadata>>,

    /// Output only. Average log probability score of the candidate.
    #[serde(rename = "avgLogprobs")]
    pub avg_logprobs: Option<Vec<f32>>,

    /// Output only. Log-likelihood scores for the response tokens and top tokens.
    #[serde(rename = "logprobsResult")]
    pub logprobs_result: Option<Vec<LogprobsResult>>,

    /// Output only. Index of the candidate in the list of response candidates.
    #[serde(rename = "index")]
    pub index: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogprobsResult {

    #[serde(rename = "topCandidates")]
    pub top_candidates: Vec<TopCandidates>,
    #[serde(rename = "chosenCandidates")]
    pub chosen_candidates: Vec<Candidates>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TopCandidates {
    pub candidates: Vec<Candidates>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Candidates {
    pub token: String,
    #[serde(rename = "tokenId")]
    pub token_id: u32,
    #[serde(rename = "logProbability")]
    pub log_probability: f32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
    pub text: String,
}

/// A collection of source attributions for a piece of content.
#[derive(Debug, Deserialize, Serialize)]
pub struct CitationMetadata {
    /// Citations to sources for a specific response.
    #[serde(rename = "citationSources")]
    pub citation_sources: Vec<CitationSource>,
}

/// A citation to a source for a portion of a specific response.
#[derive(Debug, Deserialize, Serialize)]
pub struct CitationSource {
    /// Optional. Start of segment of the response that is attributed to this source.
    /// Index indicates the start of the segment, measured in bytes.
    #[serde(rename = "startIndex")]
    pub start_index: Option<u32>,

    /// Optional. End of the attributed segment, exclusive.
    #[serde(rename = "endIndex")]
    pub end_index: Option<u32>,

    /// Optional. URI that is attributed as a source for a portion of the text.
    pub uri: Option<String>,

    /// Optional. License for the GitHub project that is attributed as a source for segment.
    /// License info is required for code citations.
    pub licence: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroundingAttribution {
    /// Output only. Identifier for the source contributing to this attribution.
    #[serde(rename = "sourceId")]
    pub source_id: Option<AttributionSourceId>,

    /// Grounding source content that makes up this attribution.
    pub content: Content
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttributionSourceId {

    #[serde(rename = "groundingPassage")]
    pub grounding_passage: GroundingPassageId,

    #[serde(rename = "semanticRetrieverChunk")]
    pub semantic_retriever_chunk: SemanticRetrieverChunk,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroundingPassageId {

    #[serde(rename = "passageId")]
    pub passage_id: Option<String>,

    #[serde(rename = "partIndex")]
    pub part_index: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SemanticRetrieverChunk {
    pub source: Option<String>,
    pub chunk: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroundingMetadata {
    #[serde(rename = "groundingChunks")]
    pub grounding_chunks: Vec<GroundingChunk>,
    #[serde(rename = "groundingSupports")]
    pub grounding_supports: Vec<GroundingSupport>,
    #[serde(rename = "webSearchQueries")]
    pub web_search_queries: String,
    #[serde(rename = "searchEntryPoint")]
    pub search_entry_point: Option<SearchEntryPoint>,
    #[serde(rename = "retrievalMetadata")]
    pub retrieval_metadata: Vec<RetrievalMetadata>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchEntryPoint {
    #[serde(rename = "renderedContent")]
    pub rendered_content: String,
    #[serde(rename = "sdkBlob")]
    pub sdk_blob: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RetrievalMetadata {
    #[serde(rename = "googleSearchDynamicRetrievalScore")]
    pub google_search_dynamic_retrieval_score: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroundingChunk {
    pub web: Web,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Web {
    pub uri: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GroundingSupport {
    #[serde(rename = "groundingChunkIndices")]
    pub grounding_chunk_indices: Vec<u32>,
    #[serde(rename = "confidenceScores")]
    pub confidence_scores: Vec<f32>,
    pub segment: Segment,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Segment {
    #[serde(rename = "partIndex")]
    pub part_index: Option<u32>,
    #[serde(rename = "startIndex")]
    pub start_index: Option<u32>,
    #[serde(rename = "endIndex")]
    pub end_index: Option<u32>,
    pub text: Option<String>,
}

/// A set of the feedback metadata the prompt specified in GenerateContentRequest.content.
#[derive(Debug, Deserialize, Serialize)]
pub struct PromptFeedback {
    /// Optional. If set, the prompt was blocked and no candidates are returned. Rephrase the prompt.
    #[serde(rename = "blockReason")]
    pub block_reason: Option<BlockReason>,

    /// Ratings for safety of the prompt. There is at most one rating per category.
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Vec<SafetyRatings>,
}

/// Safety rating for a piece of content.
#[derive(Debug, Deserialize, Serialize)]
pub struct SafetyRatings {
    /// Required. The category for this rating.
    pub category: HarmCategory,

    /// Required. The probability of harm for this content.
    pub probability: HarmProbability,

    /// Was this content blocked because of this rating?
    pub blocked: bool,
}

/// Metadata on the generation request's token usage.
#[derive(Debug, Deserialize, Serialize)]
pub struct UsageMetadata {

    /// Number of tokens in the prompt. When cachedContent is set, this is still the total effective prompt size meaning this includes the number of tokens in the cached content.
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: u32,

    /// Number of tokens in the cached part of the prompt (the cached content)
    #[serde(rename = "cachedContentTokenCount")]
    pub cached_content_token_count: Option<u32>,

    /// Total number of tokens across all the generated response candidates.
    #[serde(rename = "candidatesTokenCount", default)]
    // pub candidates_token_count: Option<u32>,
    pub candidates_token_count: u32,

    /// Output only. Number of tokens present in tool-use prompt(s).
    #[serde(rename = "toolUsePromptTokenCount")]
    pub tool_use_prompt_token_count: Option<u32>,

    /// Output only. Number of tokens of thoughts for thinking models.
    #[serde(rename = "thoughtsTokenCount")]
    pub thoughts_token_count: Option<u32>,

    /// Total token count for the generation request (prompt + response candidates).
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: u32,

    /// Output only. List of modalities that were processed in the request input.
    #[serde(rename = "promptTokensDetails", default)]
    pub prompt_tokens_details: Option<Vec<ModalityTokenCount>>,

    /// Output only. List of modalities of the cached content in the request input.
    #[serde(rename = "cachedContentTokensDetails", default)]
    pub cached_content_tokens_details: Option<Vec<ModalityTokenCount>>, 

    /// Output only. List of modalities that were returned in the response
    #[serde(rename = "candidatesTokensDetails", default)]
    pub candidates_tokens_details: Option<Vec<ModalityTokenCount>>, 

    /// Output only. List of modalities that were processed for tool-use request inputs.
    #[serde(rename = "toolUsePromptTokensDetails")]
    pub tool_use_prompt_tokens_details: Option<Vec<ModalityTokenCount>>,
}

/// Represents token counting info for a single modality.
#[derive(Debug, Deserialize, Serialize)]
pub struct ModalityTokenCount {
    /// The modality associated with this token count.
    pub modality: Modality,

    /// Number of tokens.
    #[serde(rename = "tokenCount")]
    pub token_count: u32,
}


// ============================ GEMINI API RESPONSE ENUMS =================== //

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum FinishReason {
    #[default]
    #[serde(rename = "FINISH_REASON_UNSPECIFIED")]
    FinishReasonUnspecified,	                    // Default value. This value is unused.
    #[serde(rename = "STOP")]
    Stop,	                                        // Natural stop point of the model or provided stop sequence.
    #[serde(rename = "MAX_TOKENS")]
    MaxTokens,	                                    // The maximum number of tokens as specified in the request was reached.
    #[serde(rename = "SAFETY")]
    Safety,	                                        // The response candidate content was flagged for safety reasons.
    #[serde(rename = "RECITATION")]
    Recitation,	                                    // The response candidate content was flagged for recitation reasons.
    #[serde(rename = "LANGUAGE")]
    Language,	                                    // The response candidate content was flagged for using an unsupported language.
    #[serde(rename = "OTHER")]
    Other,	                                        // Unknown reason.
    #[serde(rename = "BLOCKLIST")]
    Blocklist,	                                    // Token generation stopped because the content contains forbidden terms.
    #[serde(rename = "PROHIBITED_CONTENT")]
    ProhibitedContent,	                            // Token generation stopped for potentially containing prohibited content.
    #[serde(rename = "SPII")]
    Spii,	                                        // Token generation stopped because the content potentially contains Sensitive Personally Identifiable Information (SPII).
    #[serde(rename = "MALFORMED_FUNCTION_CALL")]
    MalformedFunctionCall,	                        // The function call generated by the model is invalid.
    #[serde(rename = "IMAGE_SAFETY")]
    ImageSafety                                     // Token generation stopped because generated images contain safety violations.
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum BlockReason {
    #[default]
    #[serde(rename = "BLOCK_REASON_UNSPECIFIED")]   // Default value. This value is unused.
    BlockReasonUnspecified,	   
    #[serde(rename = "SAFETY")]                     // Prompt was blocked due to safety reasons. Inspect safetyRatings to understand which safety category blocked it.
    Safety,  
    #[serde(rename = "OTHER")]                      // Prompt was blocked due to unknown reasons.
    Other,
    #[serde(rename = "BLOCKLIST")]                  // Prompt was blocked due to the terms which are included from the terminology blocklist.
    Blocklist,	                                    
    #[serde(rename = "PROHIBITED_CONTENT")]         // Prompt was blocked due to prohibited content.
    ProhibitedContent,	      
    #[serde(rename = "IMAGE_SAFETY")]               // Candidates blocked due to unsafe image generation content.
    ImageSafety,	                            
}

#[derive(Debug, Deserialize, Serialize)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]          // Category is unspecified.
    HarmCategoryUnspecified,
    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]           // PaLM - Negative or harmful comments targeting identity and/or protected attribute.
    HarmCategoryDerogatory,
    #[serde(rename = "HARM_CATEGORY_TOXICITY")]             // PaLM - Content that is rude, disrespectful, or profane.
    HarmCategoryToxicity,
    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]             // PaLM - Describes scenarios depicting violence against an individual or group, or general descriptions of gore.
    HarmCategoryViolence,
    #[serde(rename = "HARM_CATEGORY_SEXUAL")]               // PaLM - Contains references to sexual acts or other lewd content.
    HarmCategorySexual,
    #[serde(rename = "HARM_CATEGORY_MEDICAL")]              // PaLM - Promotes unchecked medical advice.
    HarmCategoryMedical,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]            // PaLM - Dangerous content that promotes, facilitates, or encourages harmful acts.
    HarmCategoryDangerous,
    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]           // Gemini - Harassment content.
    HarmCategoryHarassment,
    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]          // Gemini - Hate speech and content.
    HarmCategoryHateSpeech,
    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]    // Gemini - Sexually explicit content.
    HarmCategorySexuallyExplicit,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]    // Gemini - Dangerous content.
    HarmCategoryDangerousContent,
    #[serde(rename = "HARM_CATEGORY_CIVIC_INTEGRITY")]      // Gemini - Content that may be used to harm civic integrity.
    HarmCategoryCivicIntegrity,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum HarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]       // Probability is unspecified.
    HarmProbabilityUnspecified,
    #[serde(rename = "NEGLIGIBLE")]                         // Content has a negligible chance of being unsafe.
    Negligible,
    #[serde(rename = "LOW")]                                // Content has a low chance of being unsafe.
    Low,
    #[serde(rename = "MEDIUM")]                             // Content has a medium chance of being unsafe.
    Medium,
    #[serde(rename = "HIGH")]                               // Content has a high chance of being unsafe.
    High,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Modality {
    #[serde(rename = "MODALITY_UNSPECIFIED")]       // Unspecified modality.
    ModalityUnspecified,
    #[serde(rename = "TEXT")]                       // Plain text.
    Text,
    #[serde(rename = "IMAGE")]                      // Image.
    Image,
    #[serde(rename = "VIDEO")]                      // Video.
    Video,
    #[serde(rename = "AUDIO")]                      // Audio.
    Audio,
    #[serde(rename = "DOCUMENT")]                   // Document, e.g. PDF.
    Document,
}   


// ============================ GEMINI API RESPONSE Tests =================== //

#[cfg(test)]
mod tests {

    #[test]
    fn test_serialization_and_deserialization() {
        use super::*;
        let json_stop = serde_json::to_string(&FinishReason::Stop).unwrap();
        assert_eq!(json_stop, "\"STOP\"");

        let parsed_stop: FinishReason = serde_json::from_str(&json_stop).unwrap();
        match parsed_stop {
            FinishReason::Stop => (),
            _ => panic!("Deserialization failed"),
        }
    }
}