$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 6380)
$stream = $client.GetStream()

$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

$writer.Write("SET name RJ`r`n")
$writer.Flush()

Write-Host "Response:"
Write-Host $reader.ReadLine()

Write-Host "Keeping connection open for 10 seconds..."
Start-Sleep -Seconds 10

$client.Close()
