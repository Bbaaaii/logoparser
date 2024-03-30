path="./logo_examples"
for file in "$path"/*.lg; do 
    if [[ "$(basename "$file")" == "$1"* ]]; then

        echo "Testing in $file:"

        echo "Local:"
        6991 cargo run $file local.svg 500 500

        echo "Sample:"
        6991 rslogo $file sample.svg 500 500

        echo "File differences"
        diff local.svg sample.svg

        echo ""
        echo ""

    fi
done