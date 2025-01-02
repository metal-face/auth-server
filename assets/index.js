window.addEventListener("load", () => {
    let token = localStorage.getItem("token");
    if (token) {
        alert("you're logged in")
        window.location.href = "https://bryan-hughes.com"
    }
});

function createStatusMessage(success, form) {
    let messageContainer = document.createElement("div");
    let messageParagraph = document.createElement("p");
    messageParagraph.id = "message";

    if (success) {
        form[0].style.borderColor = "green";
        form[1].style.borderColor = "green";

        messageParagraph.innerText = "Success! You're now logged in!";
    } else {
        form[0].style.borderColor = "red";
        form[1].style.borderColor = "red";

        form[0].value = "";
        form[1].value = "";

        messageParagraph.innerText = "Invalid email or password";
    }

    messageContainer.appendChild(messageParagraph);
    form.appendChild(messageContainer);
}

let githubButton = document.getElementById("github");
githubButton.addEventListener("click", (e) => {
    document.getElementById("email").value = "balls";
});

let submitButton = document.getElementById("submit");
submitButton.addEventListener("click", async (e) => {
    let form = document.getElementById("credentials-form");

    if (!form[0].value || !form[1].value) {
        alert("Please enter a valid email and password");
        return;
    }

    let req = await fetch("http://localhost:6969/authenticate/credentials", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            email: form[0].value,
            password: form[1].value
        }),
    });

    let message = document.getElementById("message");
    if (message) {
        message.remove();
    }

    if (!req.ok) {
        createStatusMessage(false, form);
    }

    if (req.ok) {
        let token = await req.json();
        window.localStorage.setItem("token", token);

        createStatusMessage(true, form);
    }
})
