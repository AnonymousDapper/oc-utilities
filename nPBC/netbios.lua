--// nPBC - network Pre-Boot Controller
--// This file should be flashed to an EEPROM and used to bootload computers

local versionString = "nPBC 0.0.1-dev"

local componentInvoke = component.invoke
local invokeDevice = function(address, method, ...)
  local result = table.pack(pcall(componentInvoke, address, method, ...))
  if not result[1] then
    return nil, result[2]
  else
    return table.unpack(result, 2, result.n)
  end
end

local w, h, y = 0, 0, 1
local screen = component.list("screen")()
local gpu = component.list("gpu")()
if screen and gpu then
  invokeDevice(gpu, "bind", screen)
  w, h = invokeDevice(gpu, "maxResolution")
  invokeDevice(gpu, "setResolution", w, h)
  invokeDevice(gpu, "setBackground", 0x212121)
  invokeDevice(gpu, "setForeground", 0xCFCFFF)
  invokeDevice(gpu, "fill", 1, 1, w, h, " ")

end

local status = function(msg)
  if gpu and screen then
    invokeDevice(gpu, "set", 1, y, msg)
    if y == h then
      invokeDevice(gpu, "copy", 1, 2, w, h - 1, 0, -1)
      invokeDevice(gpu, "fill", 1, h, w, 1, " ")
    else
      y = y + 1
    end
  end
end

local bootError = function(errorMsg)
  error("[nPBC] Failed to boot: "..errorMsg, 0)
end

status("Starting pre-boot for "..versionString)

local modem = component.list("modem")()
if not modem then
  bootError("No network device")
end
status("Detected network adapter "..modem)

local eeprom = component.list("eeprom")()

computer.getBootAddress = function()
  return invokeDevice(eeprom, "getData")
end

computer.setBootAddress = function(address)
  return invokeDevice(eeprom, "setData", address)
end

status("Finished init for pre-boot")
--// Pre-boot init done

local hostAddress = ""
local fsData = ""

local setNetHost = function(address)
  hostAddress = address
  
  computer.getNPBCHostAddress = function()
    return address
  end
end

local addData = function(...)
  local data = table.pack(...)
  
  local result, reason = invokeDevice(modem, "send", address, 4011, "NPBC_DATA_ACK", data[1])
  if not result then
    bootError("Downloading boot data failed"..(reason and ": "..reason or ""))
  end
end

local finishData = function(...)
  return
end


do
 invokeDevice(modem, "open", 4011)
 local openResult, reason = invokeDevice(modem, "isOpen", 4011)
 if not openResult then
    bootError("Could not bind to port 4011"..(reason and ": "..reason or ""))
  end

 invokeDevice(modem, "broadcast", 4011, "NPBC_REQUEST_HOST")
end

status("Started search for nPBC host..")

while true do
  local data = table.pack(computer.pullSignal(30))
  if data[1] == nil then
    status("Could not find nPBC host after 30 seconds.. Halting")
    bootError("No nPBC host found")
    break
    
  end
    
  if data[1] == "modem_message" then
    if data[4] == 4011 then

      if data[6] == "NPBC_IDENTIFY_HOST" then
        setNetHost(data[3])

      elseif data[6] == "NPBC_DATA_PART" then
        addData(table.unpack(data[7]))

      elseif data[6] == "NPBC_DATA_EOF" then
        finishData(table.unpack(data[7]))
      doLoop = false
       
      end
    end
  end
end