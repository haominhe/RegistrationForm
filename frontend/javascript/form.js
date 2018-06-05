// create init

function initial() {
    $('#submit_form').click(function () {
        send_form();
    });
}


function send_form() {
    var forms = $('form');

    var inputs =  get_input(forms[0]);
    var email = inputs.email;
    delete inputs.email;
    var data =  {
        email: email,
        name: inputs.name, 
        vegetarian: inputs.vegetarian, 
        kid: inputs.kid
    };

    send_request(data);
}

function get_input(form) {
    var inputs = {};
    inputs.name = $(form).find('input[id="name"]').val();
    inputs.email = $(form).find('input[id="email"]').val();
    inputs.vegetarian = $(form).find('input[id="vegetarian"]').val();
    inputs.kid = $(form).find('input[id="kid"]').val();
    return inputs;
}

function send_request(formdata) {
    $.ajax({
        type: "POST",
        url: "http://localhost:8000/register",
        data: JSON.stringify(formdata),
        contentType: "application/json; charset=utf-8",
        success: function () {
            console.log("success");
            window.location.replace("./confirmation.html");
        }
    })
}
initial();
