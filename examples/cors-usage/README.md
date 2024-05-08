# CORS Usage Sample

This Spin App illustrates how to use CORS capabilities provided by `spin-contrib-http`. 

## Run the Spin App locally

Follow these steps to run the Spin App on your machine:

```bash
# build the Spin App
spin build

# run the Spin App
spin up
```

## Testing the Spin App

You can use `curl` to send test requests to the app. Please keep in mind that CORS headers are only send if requests specify a `Origin` header. 

Samples listed below construct resources based on the following `CorsConfig` instance:

``rust
    CorsConfig::new(
        ALL_ORIGINS.to_string(),
        ALL_METHODS.to_string(),
        ALL_HEADERS.to_string(),
        false,
        Some(3600),
    )
```

### No CORS response headers present

First, let's issue an request without an `Origin` header. (it may not contain CORS specific response headers)

```bash
curl -iX GET http://localhost:3000
HTTP/1.1 200 OK
transfer-encoding: chunked
date: Wed, 08 May 2024 14:12:36 GMT
```

### CORS Preflight

Next, let's issue a CORS preflight request (Method must be `OPTIONS` and `Origin` header must be present):

```bash
curl -iX OPTIONS -H 'Origin: http://localhost:4200'  -H 'ACCESS-CONTROL-REQUEST-METHOD: POST'  http://localhost:3000   
HTTP/1.1 405 Method Not Allowed
access-control-max-age: 3600
access-control-allow-methods: *
access-control-allow-headers: *
access-control-allow-credentials: false
access-control-allow-origin: http://localhost:4200
transfer-encoding: chunked
date: Wed, 08 May 2024 14:13:04 GMT
```


### CORS Request

An actual request with proper CORS response headers looks like this:

```bash
curl -iX GET -H 'Origin: http://localhost:4200'  http://localhost:3000
HTTP/1.1 200 OK
access-control-allow-credentials: false
access-control-allow-origin: http://localhost:4200
transfer-encoding: chunked
date: Wed, 08 May 2024 14:12:54 GMT
```