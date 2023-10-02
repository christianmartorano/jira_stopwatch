------------------
#### 🏭 Project Information:

* To run this project you need to create a file called *credentials.json* beside the $USER home directory
* The following *credentials.json* must be in the format:

```json
{"user": "", "token": "", "url": ""}
```

* The *user* is a valid es-mail user from you Jira account
* The *token* is a App created token in the Jira panel
* The *url* is a valid project URL you want to use

### 🕷️ Execute program:

* To execute the program you only need to pass the Issue ID after the program name e.g.:

```shell
./jira_stopwatch X-555
```