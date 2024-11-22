import 'package:flutter/material.dart';
import '../widgets/inputs/formInput.dart';
import '../widgets/texts/title.dart';
import '../widgets/buttons/formBtn.dart';

class LoginScreen extends StatelessWidget {
  const LoginScreen({super.key});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Login'),
        backgroundColor: const Color(0xFFD97706),
        titleTextStyle: const TextStyle(
          fontSize: 24,
          fontWeight: FontWeight.bold,
          color: Colors.white,
          fontFamily: 'Roboto',
        ),
      ),
      body: Center(
        child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16.0),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                TitleCustom(text: 'sweetmarket'),
                const SizedBox(height: 40),
                FormInput(label: 'Email', hint: 'joe@example.com'),
                const SizedBox(height: 20),
                FormInput(label: 'Password', hint: '******'),
                Row(
                  mainAxisSize: MainAxisSize.min,
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    Row(mainAxisSize: MainAxisSize.min, children: [
                      Checkbox(
                          value: false,
                          onChanged: (value) => {print('checkbox clicked')}),
                      Text('Keep me Signed in')
                    ]),
                    TextButton(
                        onPressed: () {
                          Navigator.pushNamed(context, '/forgot-password');
                        },
                        child: Text('Forgot password?'))
                  ],
                ),
                const SizedBox(height: 20),
                Formbtn(
                    content: 'Login',
                    action: () {
                      print('Login button clicked');
                    }),
                const SizedBox(height: 20),
                Formbtn(
                    content: 'Create Account',
                    action: () {
                      Navigator.pushNamed(context, '/register');
                    }),
                const SizedBox(height: 20),
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    SizedBox(
                      width: 100.0,
                      child: Container(
                        height: 1.0,
                        color: Colors.grey,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 20),
                Formbtn(
                    content: 'Sign in with Google',
                    action: () => {print('btn clicked')})
              ],
            )),
      ),
    );
  }
}
