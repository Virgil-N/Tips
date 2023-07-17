## 官方源

### homebrew源
https://github.com/Homebrew/brew.git
### homebrew-core源
https://github.com/Homebrew/homebrew-core.git
### homebrew-cask源
https://github.com/Homebrew/homebrew-cask.git

---
## 清华源

### 替换brew.git源
git -C "$(brew --repo)" remote set-url origin https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git
### 替换 homebrew-core.git源
git -C "$(brew --repo homebrew/core)" remote set-url origin https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-core.git
### 替换 homebrew-cask.git源
git -C "$(brew --repo homebrew/cask)" remote set-url origin https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-cask.git

其实替换这三个源就可以了，另外就是homebrew-bottles，这个配置个HOMEBREW_BOTTLE_DOMAIN环境变量，不过好像不配置也没啥问题：

### 使用bash的话
echo 'export HOMEBREW_BOTTLE_DOMAIN=https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles' >> ~/.bash_profile
source ~/.bash_profile

### 使用zsh的话
echo 'export HOMEBREW_BOTTLE_DOMAIN=https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles' >> ~/.zshrc
source ~/.zshrc

---

## 替换为中科大源

### brew.git源
git -C "$(brew --repo)" remote set-url origin https://mirrors.ustc.edu.cn/brew.git
### homebrew-core.git源
git -C "$(brew --repo homebrew/core)" remote set-url origin https://mirrors.ustc.edu.cn/homebrew-core.git
### homebrew-cask.git源
git -C "$(brew --repo homebrew/cask)" remote set-url origin https://mirrors.ustc.edu.cn/homebrew-cask.git
### 配置homebrew-bottles
### bash用户
echo 'export HOMEBREW_BOTTLE_DOMAIN=https://mirrors.ustc.edu.cn/homebrew-bottles' >> ~/.bash_profile
source ~/.bash_profile
### zsh用户
echo 'export HOMEBREW_BOTTLE_DOMAIN=https://mirrors.ustc.edu.cn/homebrew-bottles' >> ~/.zshrc
source ~/.zshrc

---

## 重置为官方源

### brew.git源
git -C "$(brew --repo)" remote set-url origin https://github.com/Homebrew/brew.git
### homebrew-core.git源
git -C "$(brew --repo homebrew/core)" remote set-url origin https://github.com/Homebrew/homebrew-core
### homebrew-cask.git源
git -C "$(brew --repo homebrew/cask)" remote set-url origin https://github.com/Homebrew/homebrew-cask


