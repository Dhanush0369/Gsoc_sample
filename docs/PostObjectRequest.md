# PostObjectRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expand** | Option<**bool**> | If false and the object_id refers to a bundle, then the ContentsObject array contains only those objects directly contained in the bundle. That is, if the bundle contains other bundles, those other bundles are not recursively included in the result. If true and the object_id refers to a bundle, then the entire set of objects in the bundle is expanded. That is, if the bundle contains other bundles, then those other bundles are recursively expanded and included in the result. Recursion continues through the entire sub-tree of the bundle. If the object_id refers to a blob, then the query parameter is ignored. | [optional]
**passports** | Option<**Vec<String>**> | the encoded JWT GA4GH Passport that contains embedded Visas.  The overall JWT is signed as are the individual Passport Visas. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


