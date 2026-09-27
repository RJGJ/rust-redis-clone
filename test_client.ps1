$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 6380)
$stream = $client.GetStream()

$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

function Send-Command($command) {
    $writer.Write($command + "`r`n")
    $writer.Flush()

    $response = $reader.ReadLine()

    Write-Host "> $command"
    Write-Host "< $response"
    Write-Host ""
}

Send-Command "SET name RJ"
Send-Command "GET name"
Send-Command "DEL name"
Send-Command "GET name"

$client.Close()
