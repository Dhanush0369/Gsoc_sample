# Authorizations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**supported_types** | Option<**Vec<String>**> | An Optional list of support authorization types. More than one can be supported and tried in sequence. Defaults to `None` if empty or missing. | [optional]
**passport_auth_issuers** | Option<**Vec<String>**> | If authorizations contain `PassportAuth` this is a required list of visa issuers (as found in a visa's `iss` claim) that may authorize access to this object. The caller must only provide passports that contain visas from this list. It is strongly recommended that the caller validate that it is appropriate to send the requested passport/visa to the DRS server to mitigate attacks by malicious DRS servers requesting credentials they should not have. | [optional]
**bearer_auth_issuers** | Option<**Vec<String>**> | If authorizations contain `BearerAuth` this is an optional list of issuers that may authorize access to this object. The caller must provide a token from one of these issuers. If this is empty or missing it assumed the caller knows which token to send via other means. It is strongly recommended that the caller validate that it is appropriate to send the requested token to the DRS server to mitigate attacks by malicious DRS servers requesting credentials they should not have. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


