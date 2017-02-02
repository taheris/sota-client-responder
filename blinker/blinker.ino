#define RELAY_PIN 53
#define RELAY_ON  LOW
#define RELAY_OFF HIGH

float updateFactor = 1;
boolean active = 0;
boolean blinking = 0;
unsigned long lastChanged = 0;


void setup() {
  Serial.begin(9600);
  pinMode(RELAY_PIN, OUTPUT);
  cycleRelay(2000);
}

void loop() {
  if (Serial.available() > 0) {
    setUpdateFactor(Serial.read());
  }

  if (!active) {
    return;
  }

  unsigned long now   = millis();
  unsigned long delta = now - lastChanged;

  if (blinking && delta >= 300/updateFactor) {
    digitalWrite(RELAY_PIN, RELAY_OFF);
    blinking = 0;
    lastChanged = now;
  } else if (!blinking && delta >= 500/updateFactor) {
    digitalWrite(RELAY_PIN, RELAY_ON);
    blinking = 1;
    lastChanged = now;
  }
}

void setUpdateFactor(int input) {
  switch (input) {
    case 'a':
      active = true;
      break;
    case 'z':
      active = false;
      break;
    case '1':
      updateFactor = 1;
      break;
    case '2':
      updateFactor = 2;
      break;
    case '3':
      updateFactor = 3;
      break;
    case '4':
      updateFactor = 4;
      break;
    case '5':
      updateFactor = 5;
      break;
    case '6':
      updateFactor = 6;
      break;
  }

  cycleRelay(1000);
}

void cycleRelay(int ms) {
  digitalWrite(RELAY_PIN, RELAY_ON);
  delay(ms);
  digitalWrite(RELAY_PIN, RELAY_OFF);
  blinking = 0;
  lastChanged = millis();
}
