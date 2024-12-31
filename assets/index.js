window.addEventListener("load", () => {
    console.log("loaded");

    let token = localStorage.getItem("token");
    if (token.length > 0) {
        alert("you're logged in")
        window.location.href = "https://bryan-hughes.com"
    }
});

let githubButton = document.getElementById("github");

githubButton.addEventListener("click", (e) => {
    e.preventDefault();
    document.getElementById("email").innerText = "balls";
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

    if (!req.ok) {
        form[0].style.borderColor = "red";
        form[1].style.borderColor = "red";

        form[0].value = "";
        form[1].value = "";

        const message = "Invalid email or password";
        let messageContainer = document.createElement("div");
        let messageParagraph = document.createElement("p");
        messageParagraph.innerText = message;
        messageContainer.appendChild(messageParagraph);
        form.appendChild(messageContainer);
    }

    if (req.ok) {
        let token = await req.json();
        window.localStorage.setItem("token", token);

        form[0].style.borderColor = "green";
        form[1].style.borderColor = "green";

        const message = "Success! You're now logged in!";
        let messageContainer = document.createElement("div");
        let messageParagraph = document.createElement("p");
        messageParagraph.innerText = message;
        messageContainer.appendChild(messageParagraph);
        form.appendChild(messageContainer);
    }
})
