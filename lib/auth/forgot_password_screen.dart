import 'package:flutter/material.dart';
import '../widgets/inputs/formInput.dart';
import '../widgets/texts/title.dart';
import '../widgets/buttons/formBtn.dart';
import '../widgets/boxes/navbar.dart';

class ForgotPasswordScreen extends StatelessWidget {
  const ForgotPasswordScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: Navbar(text: 'Forgot Password'),
      body: Center(
        child: Padding(
            padding: EdgeInsets.symmetric(horizontal: 16.0),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                TitleCustom(text: 'sweetmarket'),
                const SizedBox(height: 40),
                FormInput(label: 'Email', hint: 'joe@example.com'),
                const SizedBox(height: 20),
                Row(
                  mainAxisSize: MainAxisSize.min,
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    Expanded(
                      child: Formbtn(
                        content: 'Cancel',
                        action: () {
                          Navigator.pop(context);
                        },
                      ),
                    ),
                    SizedBox(width: 10), // Spacing between buttons
                    Expanded(
                      child: Formbtn(
                        content: 'Email me',
                        action: () {
                          Navigator.pushNamed(context, '/reset-password');
                        },
                      ),
                    ),
                  ],
                )
              ],
            )),
      ),
    );
  }
}
