#define X_p A0
#define X_n A1
#define Y_p A2
#define Y_n A3
#define MAX 150

#include <Servo.h>

int x = 1023;
int y = 1023;

Servo servo;

void setup() {
  Serial.begin(9600);
  servo.attach(9);
}

void loop() {
  if (!Serial.available()) {
    return;
  }
  
  char t;
  
  while(Serial.available() > 0) {
    t = Serial.read();
  }
  
  if (t != '8') {
    int n = 0;
    
    while (1) {
       while (!Serial.available());
       
       t = Serial.read();
       
       if (t == '*') {
         break;
       } 

       
       n = n * 10 + (t - '0');
    }
    
    if (n > MAX) n = MAX;
    
    servo.write(n);
    
    return;
  }
 
  pinMode(X_p, OUTPUT);
  pinMode(X_n, OUTPUT);
  pinMode(Y_p, INPUT_PULLUP);
  pinMode(Y_n, INPUT_PULLUP);
  
  digitalWrite(X_p, LOW);
  digitalWrite(X_n, LOW);
  digitalWrite(Y_p, LOW);
  digitalWrite(Y_n, LOW);
  
  digitalWrite(X_p, HIGH);
  delay(5);
  x= analogRead(Y_p);
    
  digitalWrite(X_p, LOW);
  
  delay(25);
  
  
  pinMode(X_p, INPUT_PULLUP);
  pinMode(X_n, INPUT_PULLUP);
  pinMode(Y_p, OUTPUT);
  pinMode(Y_n, OUTPUT);
  
  digitalWrite(X_p, LOW);
  digitalWrite(X_n, LOW);
  digitalWrite(Y_p, LOW);
  digitalWrite(Y_n, LOW);
  
  digitalWrite(Y_p, HIGH);
  delay(5);
  y = analogRead(X_p);
  
  digitalWrite(Y_p, LOW);
  
  Serial.print('>');
  
  if (x < 1000) {
    Serial.print(x);
    Serial.print(",");
    Serial.print(y);
  } else {
    Serial.print("0,0"); 
  }
  
  Serial.print("\n");
  
  delay(25);
}
