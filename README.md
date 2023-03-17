# Rustless

A serverless web service

Everything is included to demo deploying to Cloud Run on GCP.

Related Blogpost explaining in more detail how all this works: [https://jeanklaas.com/blog/rustless-serverless-rust/](https://jeanklaas.com/blog/rustless-serverless-rust/)

## Getting started

You can run this locally if you like by building the docker image and running it.

To build:

```
docker build . -t rustless
```

To run:

```
docker run --rm -p 8080:8080 rustless
```

## Deploying

You can deploy this by running:

```
gcloud builds submit .
```

This will kick off a cloubuild job and towards the end of the build output, you'll have a running service URL outputted in the log. It will
look something like:

```
https://rustless-3qqsqbdsuq-uc.a.run.app
```

## Contributing

Any ideas to improve or changes requested can be made via the repo's issues or pull request section and are welcome.
