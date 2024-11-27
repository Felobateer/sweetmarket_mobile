import 'package:flutter/material.dart';
import '../widgets/boxes/navbar.dart';
import '../widgets/texts/title.dart';
import '../widgets/inputs/formInput.dart';
import '../widgets/buttons/formBtn.dart';

class RegisterScreen extends StatelessWidget {
  const RegisterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: Navbar(text: 'Register'),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              TitleCustom(
                text: 'sweetmarket',
                color: Color(0xFFD97706),
              ),
              SizedBox(height: 40),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Expanded(
                    child: FormInput(label: 'Last Name', hint: 'Smith'),
                  ),
                  SizedBox(width: 16), // Add spacing between the inputs
                  Expanded(
                    child: FormInput(label: 'First Name', hint: 'Joe'),
                  ),
                ],
              ),
              SizedBox(height: 20),
              FormInput(label: 'Email', hint: 'joe@example.com'),
              SizedBox(height: 20),
              FormInput(label: 'Password', hint: '******'),
              SizedBox(height: 20),
              Formbtn(
                content: 'Register',
                action: () {
                  print('register btn clicked');
                },
              ),
            ],
          ),
        ),
      ),
    );
  }
}
