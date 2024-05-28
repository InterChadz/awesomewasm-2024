# Restaker Frontend

## Project setup

Run installations in both root and in the frontend folder:

```bash
npm install
```

## Create .env files (root / frontend)

Set the environment you want to use and choose the related compiling method described as below

```bash
cp .env.example .env
```

## Run Vue app

You must compile contracts first in order to generate ABIs that Vue.js will use to interact with them

```bash
npm run serve
```

### Customize configuration

See [Configuration Reference](https://cli.vuejs.org/config/).
