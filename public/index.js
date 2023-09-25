import init, {parse_rss} from "../pkg/rss_to_json.js"


(async () => {
    await init()

    const inputTextArea = document.getElementById("input_rss")
    const outputTextArea = document.getElementById("output_json")
    const checkboxFormat = document.getElementById("checkbox_format")

    document.getElementById("button_convert").addEventListener("click", () => {
        const rssString = inputTextArea.value
        console.log(rssString)
        outputTextArea.value = parse_rss(
            rssString,
            checkboxFormat.checked
        )
    })
})()