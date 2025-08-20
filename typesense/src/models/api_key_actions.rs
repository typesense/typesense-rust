use serde::Serialize;
use strum::{Display, EnumString};

/// Defines a single, specific action that can be granted to a Typesense API Key.
///
/// This enum provides compile-time safety and IDE autocompletion for all known
/// Typesense actions. It is marked as `#[non_exhaustive]` so that if Typesense
/// adds new actions in the future, it will not be a breaking change for your library's users.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, EnumString)]
#[non_exhaustive]
pub enum ApiKeyAction {
    // --- Collection Actions ---
    /// Allows a collection to be created. (`collections:create`)
    #[serde(rename = "collections:create")]
    #[strum(serialize = "collections:create")]
    CollectionsCreate,

    /// Allows a collection to be deleted. (`collections:delete`)
    #[serde(rename = "collections:delete")]
    #[strum(serialize = "collections:delete")]
    CollectionsDelete,

    /// Allows a collection schema to be retrieved. (`collections:get`)
    #[serde(rename = "collections:get")]
    #[strum(serialize = "collections:get")]
    CollectionsGet,

    /// Allows retrieving all collection schema. (`collections:list`)
    #[serde(rename = "collections:list")]
    #[strum(serialize = "collections:list")]
    CollectionsList,

    /// Allow all kinds of collection related operations. (`collections:*`)
    #[serde(rename = "collections:*")]
    #[strum(serialize = "collections:*")]
    CollectionsAll,

    // --- Document Actions ---
    /// Allows only search requests. (`documents:search`)
    #[serde(rename = "documents:search")]
    #[strum(serialize = "documents:search")]
    DocumentsSearch,

    /// Allows fetching a single document. (`documents:get`)
    #[serde(rename = "documents:get")]
    #[strum(serialize = "documents:get")]
    DocumentsGet,

    /// Allows creating documents. (`documents:create`)
    #[serde(rename = "documents:create")]
    #[strum(serialize = "documents:create")]
    DocumentsCreate,

    /// Allows upserting documents. (`documents:upsert`)
    #[serde(rename = "documents:upsert")]
    #[strum(serialize = "documents:upsert")]
    DocumentsUpsert,

    /// Allows updating documents. (`documents:update`)
    #[serde(rename = "documents:update")]
    #[strum(serialize = "documents:update")]
    DocumentsUpdate,

    /// Allows deletion of documents. (`documents:delete`)
    #[serde(rename = "documents:delete")]
    #[strum(serialize = "documents:delete")]
    DocumentsDelete,

    /// Allows import of documents in bulk. (`documents:import`)
    #[serde(rename = "documents:import")]
    #[strum(serialize = "documents:import")]
    DocumentsImport,

    /// Allows export of documents in bulk. (`documents:export`)
    #[serde(rename = "documents:export")]
    #[strum(serialize = "documents:export")]
    DocumentsExport,

    /// Allows all document operations. (`documents:*`)
    #[serde(rename = "documents:*")]
    #[strum(serialize = "documents:*")]
    DocumentsAll,

    // --- Alias Actions ---
    /// Allows all aliases to be fetched. (`aliases:list`)
    #[serde(rename = "aliases:list")]
    #[strum(serialize = "aliases:list")]
    AliasesList,

    /// Allows a single alias to be retrieved (`aliases:get`)
    #[serde(rename = "aliases:get")]
    #[strum(serialize = "aliases:get")]
    AliasesGet,

    /// Allows the creation of aliases. (`aliases:create`)
    #[serde(rename = "aliases:create")]
    #[strum(serialize = "aliases:create")]
    AliasesCreate,

    /// Allows the deletion of aliases. (`aliases:delete`)
    #[serde(rename = "aliases:delete")]
    #[strum(serialize = "aliases:delete")]
    AliasesDelete,

    /// Allows all alias operations. (`aliases:*`)
    #[serde(rename = "aliases:*")]
    #[strum(serialize = "aliases:*")]
    AliasesAll,

    // --- Synonym Actions ---
    /// Allows all synonyms to be fetched. (`synonyms:list`)
    #[serde(rename = "synonyms:list")]
    #[strum(serialize = "synonyms:list")]
    SynonymsList,

    /// Allows a single synonym to be retrieved (`synonyms:get`)
    #[serde(rename = "synonyms:get")]
    #[strum(serialize = "synonyms:get")]
    SynonymsGet,

    /// Allows the creation of synonyms. (`synonyms:create`)
    #[serde(rename = "synonyms:create")]
    #[strum(serialize = "synonyms:create")]
    SynonymsCreate,

    /// Allows the deletion of synonyms. (`synonyms:delete`)
    #[serde(rename = "synonyms:delete")]
    #[strum(serialize = "synonyms:delete")]
    SynonymsDelete,

    /// Allows all synonym operations. (`synonyms:*`)
    #[serde(rename = "synonyms:*")]
    #[strum(serialize = "synonyms:*")]
    SynonymsAll,

    // --- Override Actions ---
    /// Allows all overrides to be fetched. (`overrides:list`)
    #[serde(rename = "overrides:list")]
    #[strum(serialize = "overrides:list")]
    OverridesList,

    /// Allows a single override to be retrieved (`overrides:get`)
    #[serde(rename = "overrides:get")]
    #[strum(serialize = "overrides:get")]
    OverridesGet,

    /// Allows the creation of overrides. (`overrides:create`)
    #[serde(rename = "overrides:create")]
    #[strum(serialize = "overrides:create")]
    OverridesCreate,

    /// Allows the deletion of overrides. (`overrides:delete`)
    #[serde(rename = "overrides:delete")]
    #[strum(serialize = "overrides:delete")]
    OverridesDelete,

    /// Allows all override operations. (`overrides:*`)
    #[serde(rename = "overrides:*")]
    #[strum(serialize = "overrides:*")]
    OverridesAll,

    // --- Stopwords Actions ---
    /// Allows all stopword sets to be fetched. (`stopwords:list`)
    #[serde(rename = "stopwords:list")]
    #[strum(serialize = "stopwords:list")]
    StopwordsList,

    /// Allows a single stopword set to be retrieved. (`stopwords:get`)
    #[serde(rename = "stopwords:get")]
    #[strum(serialize = "stopwords:get")]
    StopwordsGet,

    /// Allows the creation of a stopword set. (`stopwords:create`)
    #[serde(rename = "stopwords:create")]
    #[strum(serialize = "stopwords:create")]
    StopwordsCreate,

    /// Allows the deletion of a stopword set. (`stopwords:delete`)
    #[serde(rename = "stopwords:delete")]
    #[strum(serialize = "stopwords:delete")]
    StopwordsDelete,

    /// Allows all stopwords operations. (`stopwords:*`)
    #[serde(rename = "stopwords:*")]
    #[strum(serialize = "stopwords:*")]
    StopwordsAll,

    // --- Keys Actions ---
    /// Allows fetching of metadata for all keys (`keys:list`)
    #[serde(rename = "keys:list")]
    #[strum(serialize = "keys:list")]
    KeysList,

    /// Allows metadata for a single key to be fetched (`keys:get`)
    #[serde(rename = "keys:get")]
    #[strum(serialize = "keys:get")]
    KeysGet,

    /// Allows the creation of API keys. (`keys:create`)
    #[serde(rename = "keys:create")]
    #[strum(serialize = "keys:create")]
    KeysCreate,

    /// Allows the deletion of API keys. (`keys:delete`)
    #[serde(rename = "keys:delete")]
    #[strum(serialize = "keys:delete")]
    KeysDelete,

    /// Allows all API Key related operations. (`keys:*`)
    #[serde(rename = "keys:*")]
    #[strum(serialize = "keys:*")]
    KeysAll,

    // --- Analytics Actions ---
    /// Allows all analytics rules and events to be fetched. (`analytics:list`)
    #[serde(rename = "analytics:list")]
    #[strum(serialize = "analytics:list")]
    AnalyticsList,

    /// Allows for a single analytics rule or event to be fetched. (`analytics:get`)
    #[serde(rename = "analytics:get")]
    #[strum(serialize = "analytics:get")]
    AnalyticsGet,

    /// Allows the creation of analytics rules and events. (`analytics:create`)
    #[serde(rename = "analytics:create")]
    #[strum(serialize = "analytics:create")]
    AnalyticsCreate,

    /// Allows the deletion of analytics rules and events. (`analytics:delete`)
    #[serde(rename = "analytics:delete")]
    #[strum(serialize = "analytics:delete")]
    AnalyticsDelete,

    /// Allows all analytics rules and events related operations. (`analytics:*`)
    #[serde(rename = "analytics:*")]
    #[strum(serialize = "analytics:*")]
    AnalyticsAll,

    // --- Analytics Rules Actions ---
    /// Allows all analytics rules to be fetched. (`analytics/rules:list`)
    #[serde(rename = "analytics/rules:list")]
    #[strum(serialize = "analytics/rules:list")]
    AnalyticsRulesList,

    /// Allows for a single analytics rule to be fetched. (`analytics/rules:get`)
    #[serde(rename = "analytics/rules:get")]
    #[strum(serialize = "analytics/rules:get")]
    AnalyticsRulesGet,

    /// Allows the creation of analytics rules. (`analytics/rules:create`)
    #[serde(rename = "analytics/rules:create")]
    #[strum(serialize = "analytics/rules:create")]
    AnalyticsRulesCreate,

    /// Allows the deletion of analytics rules. (`analytics/rules:delete`)
    #[serde(rename = "analytics/rules:delete")]
    #[strum(serialize = "analytics/rules:delete")]
    AnalyticsRulesDelete,

    /// Allows all analytics rules related operations. (`analytics/rules:*`)
    #[serde(rename = "analytics/rules:*")]
    #[strum(serialize = "analytics/rules:*")]
    AnalyticsRulesAll,

    // --- Analytics Events Actions ---
    /// Allows the creation of analytics events. (`analytics/events:create`)
    #[serde(rename = "analytics/events:create")]
    #[strum(serialize = "analytics/events:create")]
    AnalyticsEventsCreate,

    // --- Misc Actions ---
    /// Allows access to the metrics endpoint. (`metrics.json:list`)
    #[serde(rename = "metrics.json:list")]
    #[strum(serialize = "metrics.json:list")]
    MetricsJsonList,

    /// Allows access to the stats endpoint. (`stats.json:list`)
    #[serde(rename = "stats.json:list")]
    #[strum(serialize = "stats.json:list")]
    StatsJsonList,

    /// Allows access to the /debug endpoint. (`debug:list`)
    #[serde(rename = "debug:list")]
    #[strum(serialize = "debug:list")]
    DebugList,

    /// Allows all operations. (`*`)
    #[serde(rename = "*")]
    #[strum(serialize = "*")]
    All,
}
