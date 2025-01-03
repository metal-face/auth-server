window.addEventListener('load', () => {
    let token = localStorage.getItem('token');
    if (token) {
        window.location.href = 'https://bryan-hughes.com';
    }
});

let registerButton = document.getElementById('submit');
registerButton.addEventListener('click', () => {
    let form = document.forms["register"];
    let errors = document.getElementsByClassName("error");

    for (let i = 0; i < form.length; i++) {
        if (!form[i].value) {
            errors[i].innerText = "This field is required";
            return;
        }

        switch (form[i].id) {
            case "firstName":
                if (form[i].value.length < 3) {
                    errors[i].innerText = "First name must be at least 3 characters";
                }
                if (form[i].value.length > 20) {
                    errors[i].innerText = "First name must be less than 20 characters";
                }
                break;
            case "lastName":
                if (form[i].value.length < 3) {
                    errors[i].innerText = "First name must be at least 3 characters";
                }
                if (form[i].value.length > 20) {
                    errors[i].innerText = "First name must be less than 20 characters";
                }
                break;
            case "email":
                let pattern = /^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
                if (!pattern.test(form[i].value)) {
                    errors[i].innerText = "Invalid email";
                }
                break;
            case "password":
                if (form[i].value.length < 12) {
                    errors[i].innerText = "Password must be at least 12 characters";
                }
                break;
            default:
                break;
        }
    }

    let req = fetch("http://localhost:6969/users", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            firstName: form["firstName"].value,
            lastName: form["lastName"].value,
            email: form["email"].value,
            password: form["password"].value
        })
    })
});