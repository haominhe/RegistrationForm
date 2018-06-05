var total_counter = 0;

function get_guests() {
    $.ajax({
        type: "GET",
        url: "http://localhost:8000/registered",
        success: function (data) {
            display_data(data);
        },
        dataType: 'json'
    })
}

function display_data(data) {
    data.forEach(function (guest) {
        display_member(guest);
    });

    $('#total').html(`<p style="color:white; font-size:20px;">TOTAL NUMBERS OF GUESTS: ${total_counter}</p>`);
}

function display_member(guest) {
    var each_member_row =`
    <tr>
    <td>${guest.email}</td>
    <td>${guest.name}</td>
    <td>${guest.vegetarian}</td>
    <td>${guest.kid}</td>
    </tr>
    `;
   
    total_counter++;
    $('#guest_table').append($(each_member_row));
}

get_guests();
