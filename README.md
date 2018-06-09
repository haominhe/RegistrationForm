# CS510 Rust Programming - Registration Form Project


## Author: Haomin He

Copyright (c) 2018 Haomin He

### Contact: haominhe6[at]gmail[dot]com 


In this project, I want to build an event sign up form, which can be used for social events, like birthday party, graduation celebration, and wedding reception etc. In this form, guests fill up their basic personal information and dietary preferences. As a result, this form helps the host know more about guests in order to plan a good event. For instance, how many tables to arrange, how much food to order, and what kind of food to prepare.


This project requires both front end and back end. In the front end, I use HTML, JavaScript and CSS to build website structure and styling. In the back end, I use Rust for server computation and communication.


### Get Program to Run:


You may want to open the html files in your web browser: 
```
/RegistrationForm/frontend/index.html
/RegistrationForm/frontend/confirmation.html
/RegistrationForm/frontend/guests.html
http://localhost:8000/registered
```
(Note: The first two websites are used by guests for registering the event. The last two websites are used by host to display information of guests.)

In the terminal, you need to create DATABASE_URL environment variable(in the path: ```/RegistrationForm/backend```):
```
export DATABASE_URL=mysql://root:password@localhost/guests
```

Run database migration:
```
diesel migration run
```

Run command ```cargo run``` in the terminal.



### Example Screenshots:

![image](https://user-images.githubusercontent.com/13125278/41187223-b786ea14-6b59-11e8-9d59-147c9deade3a.png)


![image](https://user-images.githubusercontent.com/13125278/40683380-4be99a36-6343-11e8-8306-643bc7fc5119.png)


![image](https://user-images.githubusercontent.com/13125278/41187238-01d9c762-6b5a-11e8-83f6-5587570faf7c.png)


![image](https://user-images.githubusercontent.com/13125278/41187253-2788eb0a-6b5a-11e8-83be-214de936d9ae.png)







