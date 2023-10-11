int inches = 0;

int cm = 0;

long leedistancia(int triggerPin, int echoPin)
{

  pinMode(triggerPin, OUTPUT);  // Clear the trigger
  digitalWrite(triggerPin, LOW);
  delayMicroseconds(2);

  // Sets the trigger pin to HIGH state for 10 microseconds
 
  digitalWrite(triggerPin, HIGH);
  delayMicroseconds(10);
  digitalWrite(triggerPin, LOW);
  pinMode(echoPin, INPUT);
  
// Reads the echo pin, and returns the sound wave travel time in microseconds
  return pulseIn(echoPin, HIGH);
}

void setup()
{
  Serial.begin(115200);

}

void loop()
{
  // measure the ping time in cm
 
  cm = 0.01723 * leedistancia(A0, A1);

  // convert to inches by dividing by 2.54
  inches = (cm / 2.54);
  Serial.print(cm);
  delay(500); // Wait for 100 millisecond(s)
}
