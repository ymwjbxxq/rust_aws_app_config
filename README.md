# RUST + AWS AppConfig

In this post, I show you how to load a config dynamically and as fastest as possible.

You have two options to load your config with [AppConfig](https://docs.aws.amazon.com/appconfig/latest/userguide/appconfig-integration-lambda-extensions.html):
* Lambda Extension - this example
* Using the SDK with the [GetConfiguration](https://docs.aws.amazon.com/appconfig/2019-10-09/APIReference/API_GetConfiguration.html)

NOTE:
Not even AWS updated his page, but RUST [support it](https://awslabs.github.io/aws-sdk-rust/) and unless they will add it, there is no [example](https://github.com/awslabs/aws-sdk-rust/tree/main/examples) out there.

### AppConfig theory ###

![picture](https://github.com/ymwjbxxq/rust_aws_app_config/blob/main/readme/appconfig.png)

### AppConfig in practise ###

By default, you have 1000 TPS, and in a serverless world, it is easy to reach it.
Of course, you will not find a trace of this 1000 TPS, but you can request an increase of quota opening a ticket.

After the first request, the profile is cached, and so from now on, you hit the internal cache of AppConfig.
What I have noticed is that even if you have this in place when you reach the 1000 TPS, you will get an error like
invalid JSON response body at http://localhost:2772/applications/MyTestApplication/environments/MyTestEnvironment/configurations/MyTestProfilereason: Unexpected token R in JSON at position 0

It does not matter that you used to retry. It is like cached, and so you cannot escape this problem.
However, the retry is useful when AppConfig returns A 504 Gateway Timeout Error.

The solution is to leverage the Lambda context and call AppConfig only once until this Lambda exists:
* reducing the call to AppConfig cache
* speeding up your Lambda
* saving cents

### AppConfig hitting their CACHE ###

EXECUTION 1 - COLD START

```
START RequestId: daefa720-eab3-40d9-b982-db2f52002bfb Version: $LATEST
[appconfig agent] INFO AppConfig Lambda Extension 2.0.15
[appconfig agent] INFO serving on port 2772
EXTENSION Name: AppConfigAgent State: Ready Events: [INVOKE,SHUTDOWN]

we get the config
INFO [handler] URL "http://localhost:2772/applications/MyTestApplication/environments/MyTestEnvironment/configurations/MyTestProfile"

REPORT RequestId: daefa720-eab3-40d9-b982-db2f52002bfb Duration: 286.35
```

EXECUTION 2 - WARM STATE

```
START RequestId: 1d66614c-3272-40ec-a17b-dea81c353097 Version: $LATEST
we get the config
INFO [handler] URL "http://localhost:2772/applications/MyTestApplication/environments/MyTestEnvironment/configurations/MyTestProfile"

REPORT RequestId: 1d66614c-3272-40ec-a17b-dea81c353097 Duration: 192.21 ms 
```

EXECUTION 3 - WARM STATE

```
START RequestId: d7b1508e-e95f-4f49-9b44-f5768372a674 Version: $LATEST
we get the config
INFO [handler] URL "http://localhost:2772/applications/MyTestApplication/environments/MyTestEnvironment/configurations/MyTestProfile"

REPORT RequestId: d7b1508e-e95f-4f49-9b44-f5768372a674 Duration: 187.56 ms
```

### Using the Lambda Context ###

EXECUTION 1 - COLD START

```
START RequestId: cd5ccc81-6ff5-4fa2-8a38-e708e833cde9 Version: $LATEST
[appconfig agent] INFO AppConfig Lambda Extension 2.0.15
[appconfig agent] INFO serving on port 2772
EXTENSION Name: AppConfigAgent State: Ready Events: [INVOKE,SHUTDOWN]

we get the config
INFO [handler] URL "http://localhost:2772/applications/MyTestApplication/environments/MyTestEnvironment/configurations/MyTestProfile"

REPORT RequestId: cd5ccc81-6ff5-4fa2-8a38-e708e833cde9 Duration: 475.24
```

EXECUTION 2 - WARM STATE

```
START RequestId: d1f74efe-6ce2-4879-9374-b68535fb8e95 Version: $LATEST
we get the config
INFO [handler] URL "http://localhost:2772/applications/MyTestApplication/environments/MyTestEnvironment/configurations/MyTestProfile"

REPORT RequestId: d1f74efe-6ce2-4879-9374-b68535fb8e95 Duration: 3.79 ms
```

EXECUTION 3 - WARM STATE

```
START RequestId: 36db9855-74cf-4c3c-a560-c1baae94d96a Version: $LATEST
we get the config
INFO [handler] URL "http://localhost:2772/applications/MyTestApplication/environments/MyTestEnvironment/configurations/MyTestProfile"

REPORT RequestId: 36db9855-74cf-4c3c-a560-c1baae94d96a Duration: 3.89 ms
```

As you can see, now we drop AWS Cache Latency from over 150ms to less than 4ms.

### Build ###
```
make build
```

### Deploy ###
```
make deploy
```

### Cleanup ###
```
make delete
```
