

# API Intégrateurs EAS Cecurity TrustArch

Version: 2.0.1


# Points d'accès (Endpoints)
| Endpoint                               | Method | Action                                                                      |
|----------------------------------------| ------ |-----------------------------------------------------------------------------|
| 1 /service/authenticate                | POST   | Obtention d'un jeton d'autorisation (nécessaire pour les autres opérations) |
| 2 /eas/documents                       | POST   | Transmission d'un document -> TrustArch pour archivage                      |
| 3 /eas/documents/{ticket}              | GET    | Récupération d'une archive ou d'un document de l'archive                    |
| 4 /eas/documents/{ticket}/contentList  | GET    | Obtention de la liste détaillant le contenu d'une archive                   |
| 5 /eas/documents/{ticket}/metadata     | GET    | Obtention des métadonnées d'une archive                                     |
| 6 /eas/documents/{ticket}/metadata     | PATCH  | Mise à jour des métadonnées d'une archive                                   |
| 7 /eas/documents/{ticket}              | DELETE | Suppression d'une archive                                                   |
| 8 /eas/documents                       | GET    | Recherche d'archives                                                        |

# Exemples d'appels
# Authenticate

POST /service/authenticate

<u>Request body (application/json):</u>
```json
{
  "appId": "your application id",
  "appToken": "your application token",
  "accountName": "nom du compte déposant"
}
```
<u>Responses:</u>
- Code 200: OK
Response body (application/json):
  ```json
  {
    "token": "your authorization token"
  }
  ```
- Code 400: Bad request
  Response body (application/json):
  ```json
  {
    "Message":"Detailed error message"
  }
  ```
- Code 401: Unauthorized

# Upload a document

POST /eas/documents

<u>Request headers:</u>

- Authorization
- X-DocumentType (optional)

<u>Request body (multipart/form-data):</u>

- fingerPrint

- fingerPrintAlgorithm (NONE, MD5, SHA-1, SHA-256, SHA-512)

- fingerPrints (<b>This object overrules fingerPrint and fingerPrintAlgorithm</b>):
  [{"fileName":"file.pdf","fingerPrint":"SHA-256","fingerPrintAlgorithm":"ab54dsfg..."}]

- document (binary, support multiple files of any format)

- metadata (list of name value pairs)



<u>Responses:</u>

- Code 200: OK


Response body (application/json):

  ```json
  {
    "ticket": "unique reference towards the uploaded document"
  }
  ```

- Code 400: Bad request

  Response body (application/json):

  ```json
  {
    "Message":"Detailed error message"
  }
  ```

- Code 401: Unauthorized



<u>curl example:</u>

```shell
curl --location --request POST 'https://.../eas/documents' \
--header 'Authorization: Bearer your authorization token' \
--form 'document=@"CRA.pdf"' \
--form 'fingerprint=""' \
--form 'fingerprintAlgorithm="none"' \
--form 'metadata="[{name: \"ClientId\", value: \"1\"}, {name: \"CustomerId\", value: \"2\"}]"'
```

### Authenticate

```shell
curl --location --request POST 'https://.../EAS.INTEGRATOR.API/service/authenticate' \
--header 'Accept: application/json' \
--header 'Content-Type: application/json' \
--data-raw '{"appId":"f33c398c-0f77-4351-9f92-1e20fa3fd2f8","appToken":"e1320735-e174-4150-9edb-b5daf85be6d1","accountName":"demoAccount"}'
```

### Upload document

ClientId = 1, CustomerId = 1 and Documenttype = Invoice

```shell
curl --location --request POST 'https://.../EAS.INTEGRATOR.API/eas/documents' \
--header 'Authorization: Bearer M1ox...M0Q=' \
--form 'document=@"invoice20200201.pdf"' \
--form 'fingerprint=""' \
--form 'fingerprintAlgorithm="none"' \
--form 'metadata="[{name: \"ClientId\", value: \"1\"}, {name: \"CustomerId\", value: \"1\"},{name: \"Documenttype\", value: \"Invoice\"}]"'
```
### Get document

```shell
curl --location --request GET 'https://.../EAS.INTEGRATOR.API/eas/documents/A_FCBDAA9949D04B2980041F98E35AD05C_1' \
--header 'Accept: application/json' \
--header 'Authorization: Bearer M1ox...M0Q='
```


### Get content of an archived object

```shell
curl --location --request GET 'https://.../EAS.INTEGRATOR.API/eas/documents/A_FCBDAA9949D04B2980041F98E35AD05C_1?fileName=myFile1.pdf' \
--header 'Accept: application/json' \
--header 'Authorization: Bearer M1ox...M0Q='
```

### Get documents metadata

```shell
curl --location --request GET 'https://.../EAS.INTEGRATOR.API/eas/documents/A_FCBDAA9949D04B2980041F98E35AD05C_1/metadata' \
--header 'Accept: application/json' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer M1ox...M0Q='
```

Get documents for ClientId = 1 and Documenttype starts with Invoice

> ClientId[eq]1,Documenttype[sw]Invoice

```shell
curl --location --request GET 'https://.../EAS.INTEGRATOR.API/eas/documents?fields=ClientId,CustomerId,Documenttype&filter=ClientId[eq]1,Documenttype[sw]Inv&pageNumber=1&pageSize=20&sortBy=' \
--header 'Accept: application/json' \
--header 'Authorization: Bearer M1ox...M0Q='
```