window.addEventListener('load', () => {
    let token = localStorage.getItem('token');
    if (token) {
        window.location.href = 'https://bryan-hughes.com';
    }
});

let registerButton = document.getElementById('submit');
registerButton.addEventListener('click', () => {
    let form = document.forms["register"];

    for (let i = 0; i < form.length; i++) {
        console.log(form[i]);

        if (!form[i].value) {
            form[i].style.borderColor = "red";
        }
    }
});