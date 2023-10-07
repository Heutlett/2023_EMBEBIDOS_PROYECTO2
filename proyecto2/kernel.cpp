//
// kernel.cpp
//
// Circle - A C++ bare metal environment for Raspberry Pi
// Copyright (C) 2014-2020  R. Stange <rsta2@o2online.de>
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
#include "kernel.h"
#include <circle/string.h>
#include <circle/debug.h>
#include <assert.h>

static const char FromKernel[] = "kernel";

CKernel::CKernel (void)
:	m_Screen (m_Options.GetWidth (), m_Options.GetHeight ()),
	m_Timer (&m_Interrupt),
	m_Logger (m_Options.GetLogLevel ())
{
	m_ActLED.Blink (5);	// show we are alive
}

CKernel::~CKernel (void)
{
}

boolean CKernel::Initialize (void)
{
	boolean bOK = TRUE;

	if (bOK)
	{
		bOK = m_Screen.Initialize ();
	}
	
	if (bOK)
	{
		bOK = m_Serial.Initialize (115200);
	}
	
	if (bOK)
	{
		CDevice *pTarget = m_DeviceNameService.GetDevice (m_Options.GetLogDevice (), FALSE);
		if (pTarget == 0)
		{
			pTarget = &m_Screen;
		}

		bOK = m_Logger.Initialize (pTarget);
	}
	
	return bOK;
}


void clearBuffer(void* pBuffer, size_t nSize)
{
    char* pChar = static_cast<char*>(pBuffer);
    for (size_t i = 0; i < nSize; i++)
    {
        *pChar++ = 0;
    }
}


size_t my_strlen(const char* str)
{
    size_t len = 0;
    while (*str != '\0')
    {
        ++len;
        ++str;
    }
    return len;
}

TShutdownMode CKernel::Run (void)
{
	m_Logger.Write (FromKernel, LogNotice, "Compile time: " __DATE__ " " __TIME__);

	const char* message = "Distancia: \n";

	size_t cursorMovelength = my_strlen("\033[A\033[A");
	size_t clearLength = my_strlen("\033[J");
	size_t messageLength = my_strlen(message);

	bool flag = true; 

	while (1)
	{
		const size_t BUFFER_SIZE = 64;
        char buffer[BUFFER_SIZE];
        size_t bytesRead = 0;

        clearBuffer(buffer, BUFFER_SIZE);

		if ((bytesRead = m_Serial.Read(buffer, BUFFER_SIZE - 1)) > 0)
        {
			// Null terminate the string
            buffer[bytesRead] = '\0';

			if (!flag){
				m_Screen.Write("\033[A\033[A", cursorMovelength); // move cursor
				m_Screen.Write("\033[J", clearLength); // Clear from cursor position to end of screen
			
			} else { 
				flag = false;
			} 

			m_Screen.Write(message, messageLength);
			m_Screen.Write (buffer, BUFFER_SIZE);
			m_Screen.Write (" m", 2);
			m_Screen.Write ("\n", 1);
		}

		m_Timer.MsDelay (500);
	}


	return ShutdownHalt;
}