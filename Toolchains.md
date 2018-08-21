# Installing the different toolchains

## rust via rustup

```shell
curl https://sh.rustup.rs -sSf | sh -- --default-toolchain nightly
export PATH="$PATH:~/.cargo/bin"
```

## rbenv for ruby

```shell
git clone https://github.com/rbenv/rbenv.git ~/.rbenv
export PATH="$HOME/.rbenv/bin:$PATH"
mkdir -p "$(rbenv root)"/plugins
git clone https://github.com/rbenv/ruby-build.git "$(rbenv root)"/plugins/ruby-build
```

## nvm for node

```shell
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.8/install.sh | bash
```

## pyenv for python

```shell
git clone https://github.com/pyenv/pyenv.git ~/.pyenv
export PYENV_ROOT="$HOME/.pyenv"
export PATH="$PYENV_ROOT/bin:$PATH"
```

N.B. pyenv has made some weird design decision around what's in PATH and what can be run. https://github.com/concordusapps/pyenv-implict can help.

## The actual targets

```shell
rustup target add wasm32-unknown-unknown
rbenv install 2.5.1
pyenv install 3.5.5
nvm install 10
```
