# Iris Backend

This repo contains the backend of the Iris project.
The backend is written in Rust to ensure high performance and security.

## Building the backend

1. Ensure you have Rust installed.
2. Run `cargo build --release` to build the backend.

## Running the backend

1. Create either a config file and supply it as an argument when running the backend, 
like `iris_backend --config-file config.yaml` or set environment variables.
2. Point the Iris frontend to the backend's IP address and port, or use mDNS discovery.

## Configuration explanation

### Config file
<details>

<summary>Expand</summary>

```yaml
# Configure the backend host and port for the webserver here:
host:
port:

# Configure the tls settings here:
tls:
  enabled: false
  cert_path: ""
  key_path: ""

# Configure the engines here:
# Currently supported engines are: openai and ollama
# The maximum number of backends is 256, 
# However, configuring more than 4 engines makes no sense.
# The engines that are not used will not be loaded.
ml_engines:
  - name: my-openai
    type: openai
    url: https://api.openai.com/v1/
    api_key: ""
  - name: my-ollama
    type: ollama
    url: http://localhost:11434/
    api_key: ""

# Configure the pipeline here:
# The supported models are limited to those supported by the engines.
# The models have to be pre-loaded into the engines. Not doing so will result in an error.
# The pipeline is executed in the order of the list.
pipeline_configs:
  stt: # For transcriptions, we recommend using a model of the whisper family.
    model: whisper-large-v3-turbo
    engine_name: my-openai # The engine name from the list above.
    enabled: true # Set to false to disable the pipeline step. This may lead to unexpected behavior.
  ocr: # For optical character recognition, we recommend using either deepseek-ocr or paddleocr.
    model: deepseek-ocr
    engine_name: my-ollama
    enabled: false
  llm: # For language models, we recommend using a model supporting a large context window. 
    model: "qwen3-vl:8b" # 8b is the recommended minimum, 12b works best for most cases.
    engine_name: my-ollama
    vision_model: true # Vision models are very powerful and often don't require the previous ocr step. 
                       # However, they take considerably longer for the inference.
    enabled: true
  tts:
    model: "qwen3-tts" # We achieved the best quality with this model. It is, however, not the fastest.
    engine_name: my-openai
    enabled: true
```

</details>

### Environment variables

<details>
<summary>Expand</summary>

```dotenv
# Configure the backend host and port for the webserver here:
IRIS_HOST=
IRIS_PORT=

# Configure the tls settings here:
IRIS_TLS_ENABLED=false
IRIS_TLS_KEY_FILE=
IRIS_TLS_CERT_FILE=

# Configure the engines here:
# The same recommandations as in the config file apply here.
# The numer of engines must explciitly be set in contrast to the config file.
IRIS_ML_ENGINES_COUNT=2

# Engine configuration no. 1 
IRIS_ML_ENGINE_0_NAME=my-openai
IRIS_ML_ENGINE_0_TYPE=openai
IRIS_ML_ENGINE_0_URL=https://api.openai.com/v1
IRIS_ML_ENGINE_0_API_KEY=""

# Engine configuration no. 2
IRIS_ML_ENGINE_1_NAME=my-ollama
IRIS_ML_ENGINE_1_TYPE=ollama
IRIS_ML_ENGINE_1_URL=http://localhost:11434
IRIS_ML_ENGINE_1_API_KEY=""

# Configure the pipeline here:
# Details are the same as in the config file.
IRIS_PIPELINE_STT_MODEL=whisper-large-v3-turbo
IRIS_PIPELINE_STT_ENGINE_NAME=my-openai
IRIS_PIPELINE_STT_ENGINE_ENABLED=true

IRIS_PIPELINE_OCR_MODEL=deepseek-ocr
IRIS_PIPELINE_OCR_ENGINE_NAME=my-ollama
IRIS_PIPELINE_OCR_ENGINE_ENABLED=true

IRIS_PIPELINE_LLM_MODEL="qwen3-vl:8b"
IRIS_PIPELINE_LLM_ENGINE_NAME=my-ollama
IRIS_PIPELINE_LLM_VISION_VISION_MODEL=true
IRIS_PIPELINE_LLM_ENGINE_ENABLED=true

IRIS_PIPELINE_TTS_MODEL="qwen3-tts"
IRIS_PIPELINE_TTS_ENGINE_NAME=my-openai
IRIS_PIPELINE_TTS_ENGINE_ENABLED=true
```

</details>

## Project Status

- [x] Configuration via config file
- [x] Configuration via environment variables
- [x] Logging
- [ ] mDNS discovery
- [ ] Websocket handling
- [ ] Pipeline execution and management
- [ ] Unit tests (currently incomplete)


## Contributing

This project is still in its early stages, and the main contributors are currently studying at the 
University of Applied Sciences in Ulm (THU). This project is the required student project for the 
Bachelor of Engineering in Electronics Engineering at THU.

We are open to contributions, and we welcome any feedback or suggestions.
