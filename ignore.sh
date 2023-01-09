#! /bin/zsh

echo "# This .gitignore file is auto-generated. Do not edit!" > .gitignore
echo "# Edit Ignorefile instead." >> .gitignore
echo "" >> .gitignore
echo "" >> .gitignore


num=$(yq '."gitignore.io" | length' Ignorefile)
for i in $(seq 0 $(($num-1))); do
    target=$(yq ".\"gitignore.io\".[$i]" Ignorefile)

    curl -L -s "https://www.gitignore.io/api/$target" >> .gitignore || exit 2

    echo "" >> .gitignore
    echo "" >> .gitignore
    echo "" >> .gitignore
done

num=$(yq '.github | length' Ignorefile)
for i in $(seq 0 $(($num-1))); do
    target=$(yq ".github.[$i]" Ignorefile)

    url="https://raw.githubusercontent.com/github/gitignore/main/$target.gitignore"
    
    echo "# Downloaded from $url" >> .gitignore
    echo "" >> .gitignore
    curl -L -s $url >> .gitignore || exit 2

    echo "" >> .gitignore
    echo "" >> .gitignore
    echo "" >> .gitignore
done

echo "### Project specific settings ###" >> .gitignore
echo "" >> .gitignore

num=$(yq '.paths-ignore | length' Ignorefile)
for i in $(seq 0 $(($num-1))); do
    projectspecific=$(yq ".paths-ignore.[$i]" Ignorefile)

    echo $projectspecific >> .gitignore
done
