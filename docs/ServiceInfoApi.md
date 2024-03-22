# \ServiceInfoApi

All URIs are relative to *https://drs.example.org/ga4gh/drs/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_service_info**](ServiceInfoApi.md#get_service_info) | **GET** /service-info | Retrieve information about this service



## get_service_info

> models::GetServiceInfo200Response get_service_info()
Retrieve information about this service

Returns information about the DRS service  Extends the [v1.0.0 GA4GH Service Info specification](https://github.com/ga4gh-discovery/ga4gh-service-info) as the standardized format for GA4GH web services to self-describe.  According to the  [service-info type registry](https://github.com/ga4gh/TASC/blob/master/service-info/ga4gh-service-info.json) maintained by the [Technical Alignment Sub Committee (TASC)](https://github.com/ga4gh/TASC), a DRS service MUST have:   * a `type.group` value of `org.ga4gh`   * a `type.artifact` value of `drs`  e.g. ``` {     \"id\": \"com.example.drs\",     \"description\": \"Serves data according to DRS specification\",     ...     \"type\": {         \"group\": \"org.ga4gh\",         \"artifact\": \"drs\"     }     ... } ```  See the [Service Registry Appendix](#tag/GA4GH-Service-Registry) for more information on how to register a DRS service with a service registry.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetServiceInfo200Response**](GetServiceInfo_200_response.md)

### Authorization

[BasicAuth](../README.md#BasicAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

