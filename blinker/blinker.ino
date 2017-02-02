#define SERIAL_BAUD 9600

#define RELAY_PIN 53
#define RELAY_ON  LOW
#define RELAY_OFF HIGH

#define TIME_SEQ  5000
#define TIME_ON   1000
#define TIME_OFF  2000
#define TIME_DOT  1000
#define TIME_DASH 2000
#define TIME_WAIT 300

#define SEQUENCE_START "--.."
#define SEQUENCE_PARSE ".."
#define SEQUENCE_SOS   "...---..."

float         timeDivisor = 1;
boolean       active      = false;
boolean       blinking    = false;
char*         sequence    = NULL;
unsigned long lastTime    = 0;


void setup() {
  Serial.begin(SERIAL_BAUD);
  pinMode(RELAY_PIN, OUTPUT);
  morseSequence(SEQUENCE_START);
}

void loop() {
  if (Serial.available() > 0) {
    parseChar(Serial.read());
  }
  if (!active) {
    return;
  }

  unsigned long timeDelta = millis() - lastTime;
  if (sequence && timeDelta >= TIME_SEQ/timeDivisor) {
    morseSequence(sequence);
  } else if (sequence) {
  } else if (blinking && timeDelta >= TIME_ON/timeDivisor) {
    blinkersOff();
  } else if (!blinking && timeDelta >= TIME_OFF/timeDivisor) {
    blinkersOn();
  }
}

void parseChar(char c) {
  sequence = NULL;

  switch (c) {
    default:  active = false;
    case 'a': active = true;
    case '1': timeDivisor = 1;
    case '2': timeDivisor = 2;
    case '3': timeDivisor = 3;
    case '4': timeDivisor = 4;
    case '5': timeDivisor = 5;
    case '6': timeDivisor = 6;
    case '7': timeDivisor = 7;
    case '8': timeDivisor = 8;
    case '9': timeDivisor = 9;
    case 's': sequence = SEQUENCE_SOS;
  }

  morseSequence(SEQUENCE_PARSE);
}

void blinkersOn() {
  digitalWrite(RELAY_PIN, RELAY_ON);
  blinking = true;
  lastTime = millis();
}

void blinkersOff() {
  digitalWrite(RELAY_PIN, RELAY_OFF);
  blinking = false;
  lastTime = millis();
}

void flashBlinkers(int ms) {
  digitalWrite(RELAY_PIN, RELAY_ON);
  delay(ms);
  digitalWrite(RELAY_PIN, RELAY_OFF);
  blinking = false;
  lastTime = millis();
}

void morseDot() {
  flashBlinkers(TIME_DOT/timeDivisor);
  delay(TIME_WAIT/timeDivisor);
}

void morseDash() {
  flashBlinkers(TIME_DASH/timeDivisor);
  delay(TIME_WAIT/timeDivisor);
}

void morseSequence(char *sequence) {
  char c;
  while ((c = *sequence++)) {
    switch (c) {
    case '.': morseDot();
    case '-': morseDash();
    }
  }
}
