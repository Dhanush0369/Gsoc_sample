# AccessMethod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of the access method. | 
**access_url** | Option<[**models::AccessMethodAccessUrl**](AccessMethod_access_url.md)> |  | [optional]
**access_id** | Option<**String**> | An arbitrary string to be passed to the `/access` method to get an `AccessURL`. This string must be unique within the scope of a single object. Note that at least one of `access_url` and `access_id` must be provided. | [optional]
**region** | Option<**String**> | Name of the region in the cloud service provider that the object belongs to. | [optional]
**authorizations** | Option<[**models::AccessMethodAuthorizations**](AccessMethod_authorizations.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


