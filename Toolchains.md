# Installing the different toolchains

## rust via rustup
curl https://sh.rustup.rs -sSf | sh
export PATH="$PATH:~/.cargo/bin"

## rbenv for ruby
git clone https://github.com/rbenv/rbenv.git ~/.rbenv
export PATH="$HOME/.rbenv/bin:$PATH"
mkdir -p "$(rbenv root)"/plugins
git clone https://github.com/rbenv/ruby-build.git "$(rbenv root)"/plugins/ruby-build

## nvm for node
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.8/install.sh | bash

## pyenv for python
git clone https://github.com/pyenv/pyenv.git ~/.pyenv
export PYENV_ROOT="$HOME/.pyenv"
export PATH="$PYENV_ROOT/bin:$PATH"

## The actual targets
rustup target add wasm32-unknown-unknown
rbenv install 2.5.1
pyenv install 3.5.5
nvm install 8
