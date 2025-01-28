# promptx

[![Actions Status](https://github.com/ai-flowx/promptx/workflows/ci/badge.svg?branch=main&event=push)](https://github.com/ai-flowx/promptx/actions?query=workflow%3Aci)
[![License](https://img.shields.io/github/license/ai-flowx/promptx.svg?color=brightgreen)](https://github.com/ai-flowx/promptx/blob/main/LICENSE)
[![Tag](https://img.shields.io/github/tag/ai-flowx/promptx.svg?color=brightgreen)](https://github.com/ai-flowx/promptx/tags)



## Introduction

*promptx* is the ai prompt generation of [flowx](https://github.com/ai-flowx/) written in Rust.



## Prerequisites

- Rust >= 1.83.0



## Run

```bash
```



## Usage

```
Usage: promptx --config-file <FILE>

Options:
  -c, --config-file <FILE>  Config file [default: config.yml]
  -h, --help                Print help
  -V, --version             Print version
```



## Settings

*promptx* parameters can be set in the directory [config](https://github.com/ai-flowx/promptx/blob/main/src/config).

An example of configuration in [config.yml](https://github.com/ai-flowx/promptx/blob/main/src/config/config.yml):

```yaml
llm:
  - name: doubao
    api: https://ark.cn-beijing.volces.com/api/v3/chat/completions
    key: 8429f8ab-*
    endpoint: ep-*
  - name: openai
    api: https://api.openai.com/v1/chat/completions
    key: 9429f8ab-*
    endpoint: ep-*
```



## Android

```bash
export ANDROID_NDK_ROOT=/path/to/android_ndk_root
cargo ndk -t aarch64-linux-android build --release
```



## License

Project License can be found [here](LICENSE).



## Reference

- [aosp-promptx](https://android-review.googlesource.com/c/platform/manifest/+/3456966)
- [bindings-to-openssl](https://docs.rs/openssl/latest/openssl/)
- [promptfoo](https://github.com/promptfoo/promptfoo)
- [prompthancer](https://prompthancer.com/)
- [promptperfect](https://promptperfect.jina.ai/)
- [prompttools](https://github.com/hegelai/prompttools)
- [promptwizard](https://github.com/microsoft/PromptWizard)
- [promptwizard](https://github.com/craftslab/promptwizard)
- [textpod](https://github.com/freetonik/textpod)
