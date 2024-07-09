/*
 *  Project     Macro Keyboard
 *  @author     Felipe Furst
 *  @link       
 *  @license    MIT - Copyright (c) 2024 Felipe Furst
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 *
 */
 

#define BUTTON_MAP_0 '1'
#define BUTTON_MAP_1 '2'
#define BUTTON_MAP_2 '3'
#define BUTTON_MAP_3 '4'
#define BUTTON_MAP_4 '5'
#define BUTTON_MAP_5 '6'
#define BUTTON_MAP_6 '7'
#define BUTTON_MAP_7 '8'
#define BUTTON_MAP_8 '9'
#define BUTTON_MAP_9 '0'
#define BUTTON_MAP_10 'a'
#define BUTTON_MAP_11 'b'
#define BUTTON_MAP_12 'c'
#define BUTTON_MAP_13 'd'
#define BUTTON_MAP_14 'e'
#define BUTTON_MAP_15 'f'
#define BUTTON_MAP_16 'g'
#define BUTTON_MAP_17 'h' 
 

#include "HID-Project-RawHID.h" // I need thust the RawHID part
#include "Keypad.h"
#include "Keyboard.h"

#include <EEPROM.h>


// ATTENTION TO THE WAY YOU WIRE AND THE DIODES DIRECTION 
const byte ROWS = 6; 
const byte COLS = 3; 
char keys[ROWS][COLS] = {
  {BUTTON_MAP_0, BUTTON_MAP_6, BUTTON_MAP_12},
  {BUTTON_MAP_1, BUTTON_MAP_7, BUTTON_MAP_13},
  {BUTTON_MAP_2, BUTTON_MAP_8, BUTTON_MAP_14},
  {BUTTON_MAP_3, BUTTON_MAP_9, BUTTON_MAP_15},
  {BUTTON_MAP_4, BUTTON_MAP_10,BUTTON_MAP_16},
  {BUTTON_MAP_5, BUTTON_MAP_11,BUTTON_MAP_17},
};
byte rowPins[ROWS] = {16,3,4,5,6,7}; // Does it seem backwards ?
byte colPins[COLS] = {8,9,10};       // That's right, I soldered the diodes in the wrong direction

Keypad kpd = Keypad( makeKeymap(keys), rowPins, colPins, ROWS, COLS );

uint8_t combination[18][5] = {
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '1'},
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '2'},
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '3'},
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '4'},
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '5'},
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '6'},
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '7'},
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '8'},
  { 0x00, 0x00, 0x00, 0x00, 0x00},
  { 0x00, 0x00, 0x00, 0x00, 0x00},
  { 0x00, 0x00, 0x00, 0x00, 0x00},
  { 0x00, 0x00, 0x00, 0x00, 0x00},
  { 0x00, 0x00, 0x00, 0x00, KEY_LEFT_ARROW},
  { 0x00, 0x00, 0x00, 0x00, KEY_RIGHT_ARROW},
  { 0x00, 0x00, 0x00, 0x00, 0x00},
  { 0x00, 0x00, 0x00, 0x00, 0x00},
  { 0x00, 0x00, 0x00, 0x00, 0x00},
  { 0x00, KEY_LEFT_CTRL, KEY_LEFT_SHIFT, KEY_LEFT_ALT, '0'},
};

int pressedKeys[22] = {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
 
const uint8_t ledPin = 17;

uint8_t rawhidData[255];

void setup() { 
  Serial.begin(9600);

  // hold yer horses... if needed
  pinMode(1, INPUT_PULLUP);
  if(!digitalRead(1)){
    failsafe();
  }

  checkEEPROM();
  loadConfig();

  RawHID.begin(rawhidData, sizeof(rawhidData));

  Keyboard.begin();
  Keyboard.releaseAll();

  kpd.setDebounceTime(30);
  
  TXLED0;
  RXLED0;

  // Set LEDs Off. Active low.
  pinMode(ledPin, OUTPUT);
  digitalWrite(ledPin, HIGH);  

}
 
void loop() {
  if (kpd.getKeys()) {
    for (int i=0; i<LIST_MAX; i++) {
      if ( kpd.key[i].stateChanged ) {
        switch ( kpd.key[i].kstate ) {  
          case PRESSED:                  
            for(int k=0; k<5; k++) {
              if(combination[findKey(kpd.key[i].kchar)][k] != 0x00) {
                if(k < 4) {
                  if(pressedKeys[k] <= 0) {
                    Keyboard.press(combination[findKey(kpd.key[i].kchar)][k]);
                  }
                  pressedKeys[k] = pressedKeys[k] + 1;
                }
                else {
                  int keyidx = findKey(kpd.key[i].kchar) + 4;
                  if(pressedKeys[keyidx] <= 0) {
                    Keyboard.press(combination[findKey(kpd.key[i].kchar)][k]);

                    digitalWrite(ledPin, LOW);
                  }
                  pressedKeys[keyidx] = pressedKeys[keyidx] + 1;
                }
              }
            }
          default:
          break;
        }
      }
    }

    for (int i=0; i<LIST_MAX; i++) {
      if ( kpd.key[i].stateChanged ) {
        switch (kpd.key[i].kstate) {  
          case RELEASED:            
            for(int k=0; k<5; k++) {
              if(combination[findKey(kpd.key[i].kchar)][k] != 0x00) {
                if(k < 4) {
                  pressedKeys[k] = pressedKeys[k] - 1;
                  
                  if(pressedKeys[k] <= 0) {
                    Keyboard.release(combination[findKey(kpd.key[i].kchar)][k]);
                  }
                }
                else {
                  int keyidx = findKey(kpd.key[i].kchar) + 4;
                  pressedKeys[keyidx] = pressedKeys[keyidx] - 1;

                  if(pressedKeys[keyidx] <= 0) {
                    Keyboard.release(combination[findKey(kpd.key[i].kchar)][k]);

                    digitalWrite(ledPin, HIGH);
                  }
                }       
              }
            }
          default:
          break;
        }
      }
    }
  }
  hidCheckData();
}
 

enum ECommand : uint8_t {
  WRITE_KEY   = 0xCC,
  READ_KEY    = 0xCB,
};
 

void hidCheckData() {
  auto bytesAvailable = RawHID.available();
  if (bytesAvailable)
  {
    uint8_t writeParam[6];
    int writeParamCount = 0;
    uint8_t readParam[1];
    int readParamCount = 0;

    uint8_t command=0x00;
    
    while (bytesAvailable--) {
      uint8_t read = RawHID.read();

      switch(command) {
        case WRITE_KEY:
          if(writeParamCount<6) {
            writeParam[writeParamCount++] = read;
          }
          break;
        case READ_KEY:
          if(readParamCount<1){
            readParam[readParamCount++] = read;
          }
          break;
      }

      if(command == 0x00) {
        command = read;
      }
    }

    switch(command) {
      case WRITE_KEY:
        writeConfig(writeParam);
        break;
      case READ_KEY:
        sendConfig(readParam);
        break;
    }

  }
}


void checkEEPROM() {
  Serial.print("Checking eeprom... ");
  char test[5] = {'m','a','c','r','o'};
  for(int i=0; i<5; i++) {
    if(test[i] != EEPROM.read(i)) {
      
      Serial.print("Formating eeprom... ");
      Serial.print(EEPROM.length());
      Serial.println(" bytes");

      for (int j = 0 ; j < EEPROM.length() ; j++) {
        if(j<5) {
          EEPROM.write(j, test[j]);
        }
        else {
          EEPROM.write(j, 0x00);
        }
      }
    }
  }
}

void loadConfig() {
  Serial.println("Loading config...");
  int total = (5*18);
  for(int i=0; i<total; i++) {
    combination[int(floor(i/5))][int(floor(i%5))] = EEPROM.read(i + 5);
  }
}

void writeConfig(uint8_t config[6]) {
  Serial.println("Writing config...");
  for(int i=1; i<6; i++) {
    combination[config[0]][i-1] = config[i];
    int idx = (config[0]*5)+(i-1);
    EEPROM.write(idx+5, config[i]);
  }
}

void sendConfig(uint8_t config[1]) {
  if(config[0] > -1 && config[0] < 18) {
    for(int i=0; i<5; i++) {
      uint8_t data = combination[config[0]][i];
      RawHID.write(data);
    }
  }
}


int findKey(char chr) {
  int count = 0;
  for(int i=0; i<COLS; i++) {
    for(int j=0; j<ROWS; j++) {
      if(keys[j][i] == chr) {
        return count;
      }
      count++;
    }
  }
  return -1;
}

void failsafe(){
  for(;;){}
}