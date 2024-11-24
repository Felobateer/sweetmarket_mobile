import 'package:flutter/material.dart';
import '../widgets/boxes/navbar.dart';
import '../widgets/texts/title.dart';
import '../widgets/inputs/formInput.dart';
import '../widgets/buttons/formBtn.dart';

class ResetPasswordScreen extends StatelessWidget {
  const ResetPasswordScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: Navbar(text: 'Reset Password'),
      body: Center(
          child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16.0),
              child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    TitleCustom(text: 'sweetmarket'),
                    SizedBox(height: 40),
                    FormInput(label: 'Email', hint: 'joe@example.com'),
                    SizedBox(height: 20),
                    FormInput(label: 'Password', hint: '******'),
                    SizedBox(height: 20),
                    FormInput(label: 'Confirm Password', hint: '******'),
                    SizedBox(height: 20),
                    Formbtn(
                        content: 'Confirm',
                        action: () {
                          Navigator.popUntil(context, ModalRoute.withName('/'));
                        })
                  ]))),
    );
  }
}
