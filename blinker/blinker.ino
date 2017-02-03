#define SERIAL_BAUD 9600

#define RELAY_PIN 53
#define RELAY_ON  LOW
#define RELAY_OFF HIGH

#define TIME_WAIT  3000
#define TIME_ON    500
#define TIME_OFF   300
#define TIME_DOT   500
#define TIME_DASH  1000
#define TIME_MORSE 300

#define SEQ_WAIT  "w"
#define SEQ_BLINK "b"
#define SEQ_SOS   "...---...w"

char* sequence = SEQ_WAIT;
float speed    = 1;


void setup() {
  Serial.begin(SERIAL_BAUD);
  pinMode(RELAY_PIN, OUTPUT);
  ack();
}

void loop() {
  if (Serial.available() > 0) {
    parseChar(Serial.read());
  }

  char* c = sequence;
  while (*c) {
    switch (*c++) {
    case 'w': wait();      break;
    case 'b': blink();     break;
    case '.': morseDot();  break;
    case '-': morseDash(); break;
    }
  }
}


void parseChar(char c) {
  switch (c) {
  case 'w': sequence = SEQ_WAIT;  break;
  case 'b': sequence = SEQ_BLINK; break;
  case 's': sequence = SEQ_SOS;   break;
  case '1': speed = 1; break;
  case '2': speed = 2; break;
  case '3': speed = 3; break;
  case '4': speed = 4; break;
  case '5': speed = 5; break;
  case '6': speed = 6; break;
  case '7': speed = 7; break;
  case '8': speed = 8; break;
  case '9': speed = 9; break;
  default : return;
  }
  ack();
}

void ack() {
  delay(200);
  digitalWrite(RELAY_PIN, RELAY_ON);
  delay(2000);
  digitalWrite(RELAY_PIN, RELAY_OFF);
  delay(200);
}

void wait() {
  digitalWrite(RELAY_PIN, RELAY_OFF);
  delay(TIME_WAIT/speed);
}

void blink() {
  digitalWrite(RELAY_PIN, RELAY_ON);
  delay(TIME_ON/speed);
  digitalWrite(RELAY_PIN, RELAY_OFF);
  delay(TIME_OFF/speed);
}

void morseDot() {
  digitalWrite(RELAY_PIN, RELAY_ON);
  delay(TIME_DOT/speed);
  digitalWrite(RELAY_PIN, RELAY_OFF);
  delay(TIME_MORSE/speed);
}

void morseDash() {
  digitalWrite(RELAY_PIN, RELAY_ON);
  delay(TIME_DASH/speed);
  digitalWrite(RELAY_PIN, RELAY_OFF);
  delay(TIME_MORSE/speed);
}
