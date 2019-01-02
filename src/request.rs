#[derive(Debug)]
pub enum RequestType {
    Word,
    Definitions,
    Synonyms,
    Antonyms,
    Examples,
    Rhymes,
    Frequency,
    IsATypeOf,
    HasTypes,
    PartOf,
    HasParts,
    IsAnInstanceOf,
    HasInstances,
    InRegion,
    RegionOf,
    UsageOf,
    HasUsages,
    IsAMemberOf,
    HasMembers,
    IsASubstanceOf,
    HasSubstances,
    HasAttribute,
    InCategory,
    HasCategories,
    Also,
    PertainsTo,
    SimilarTo,
    Entails,
}

pub trait HasRequestType {
    fn request_type() -> RequestType;
}
