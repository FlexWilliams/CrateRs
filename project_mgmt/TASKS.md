# CrateRs - Tasks

[Back to Project Board](./BOARD.md)


## CRS-001 - Create placeholder api endpoints for remoteStorage

### Description:

As a client of CrateRs, I want to be able to call the available remoteStorage spec API endpoints.

For now no auth or implementation needed, just return 2xx statuses for each.


```
 The actions the interface exposes are:

       *  GET a folder: retrieve the names and current versions of the
          documents and subfolders currently contained by the folder
       *  GET a document: retrieve its content type, current version,
          and contents

       *  PUT a document: store a new version, its content type, and
          contents, conditional on the current version

       *  DELETE a document: remove it from the storage, conditional on
          the current version

       *  HEAD a folder or document: like GET, but omitting the response
          body
```


See [here for the latest spec](https://github.com/remotestorage/spec/blob/main/release/draft-dejong-remotestorage-22.txt)

### Acceptance Criteria:

^^ ( same as description)

Verified By `<>` on: ``

<br>

## CRS-002 - Integrate oauth into CrateRs

### Description:

As a client of CrateRs, I need to initiate an oauth flow to get an access token which will allow me to make authenticaed CRUD operations.

Dev notes
1. May require self-signed ssl cert for local https development

Libraries:
1. https://docs.rs/oauth2/latest/oauth2/
2. https://oauth.net/code/rust/

### Acceptance Criteria:

^^ ( same as description)

Verified By `<>` on: ``

<br>

## CRS-003 - Implement API (i.e., directory/file/object operations)

### Description:

Implement API endpoints created [here](#crs-001---create-placeholder-api-endpoints-for-remotestorage).

### Acceptance Criteria:

^^ ( same as description)

Verified By `<>` on: ``

<br>

## CRS-004 - Dockerize CrateRs

### Description:

Do the needful of dockerizing por favor (i.e. create container deployable to fly.io)

### Acceptance Criteria:

^^ ( same as description)

Verified By `<>` on: ``

<br>

## CRS-005 - Implement PUT/DELETE file restriction

### Description:

If the path ends in a `/`, then the path is considered a folder and cannot be added/deleted, only files.

Return a 400 with the explanation as to why.

### Acceptance Criteria:

^^ ( same as description)

Verified By `<>` on: ``

<br>