# \ObjectsApi

All URIs are relative to *https://drs.example.org/ga4gh/drs/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_access_url**](ObjectsApi.md#get_access_url) | **GET** /objects/{object_id}/access/{access_id} | Get a URL for fetching bytes
[**get_object**](ObjectsApi.md#get_object) | **GET** /objects/{object_id} | Get info about a DrsObject.
[**options_object**](ObjectsApi.md#options_object) | **OPTIONS** /objects/{object_id} | Get Authorization info about a DrsObject.
[**post_access_url**](ObjectsApi.md#post_access_url) | **POST** /objects/{object_id}/access/{access_id} | Get a URL for fetching bytes through POST'ing a Passport
[**post_object**](ObjectsApi.md#post_object) | **POST** /objects/{object_id} | Get info about a DrsObject through POST'ing a Passport.



## get_access_url

> models::AccessUrl get_access_url(object_id, access_id)
Get a URL for fetching bytes

Returns a URL that can be used to fetch the bytes of a `DrsObject`. This method only needs to be called when using an `AccessMethod` that contains an `access_id` (e.g., for servers that use signed URLs for fetching object bytes).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | `DrsObject` identifier | [required] |
**access_id** | **String** | An `access_id` from the `access_methods` list of a `DrsObject` | [required] |

### Return type

[**models::AccessUrl**](AccessURL.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object

> models::DrsObject get_object(object_id, expand)
Get info about a DrsObject.

Returns object metadata, and a list of access methods that can be used to fetch object bytes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | `DrsObject` identifier | [required] |
**expand** | Option<**bool**> | If false and the object_id refers to a bundle, then the ContentsObject array contains only those objects directly contained in the bundle. That is, if the bundle contains other bundles, those other bundles are not recursively included in the result. If true and the object_id refers to a bundle, then the entire set of objects in the bundle is expanded. That is, if the bundle contains other bundles, then those other bundles are recursively expanded and included in the result. Recursion continues through the entire sub-tree of the bundle. If the object_id refers to a blob, then the query parameter is ignored. |  |

### Return type

[**models::DrsObject**](DrsObject.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## options_object

> models::Authorizations options_object(object_id)
Get Authorization info about a DrsObject.

Returns a list of `Authorizations` that can be used to determine how to authorize requests to `GetObject` or `PostObject`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | `DrsObject` identifier | [required] |

### Return type

[**models::Authorizations**](Authorizations.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_access_url

> models::AccessUrl post_access_url(object_id, access_id, post_access_url_request)
Get a URL for fetching bytes through POST'ing a Passport

Returns a URL that can be used to fetch the bytes of a `DrsObject`. This method only needs to be called when using an `AccessMethod` that contains an `access_id` (e.g., for servers that use signed URLs for fetching object bytes). Method is a POST to accommodate a JWT GA4GH Passport sent in the formData in order to authorize access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | `DrsObject` identifier | [required] |
**access_id** | **String** | An `access_id` from the `access_methods` list of a `DrsObject` | [required] |
**post_access_url_request** | [**PostAccessUrlRequest**](PostAccessUrlRequest.md) |  | [required] |

### Return type

[**models::AccessUrl**](AccessURL.md)

### Authorization

[PassportAuth](../README.md#PassportAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object

> models::DrsObject post_object(object_id, post_object_request)
Get info about a DrsObject through POST'ing a Passport.

Returns object metadata, and a list of access methods that can be used to fetch object bytes. Method is a POST to accommodate a JWT GA4GH Passport sent in the formData in order to authorize access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | `DrsObject` identifier | [required] |
**post_object_request** | [**PostObjectRequest**](PostObjectRequest.md) |  | [required] |

### Return type

[**models::DrsObject**](DrsObject.md)

### Authorization

[PassportAuth](../README.md#PassportAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

