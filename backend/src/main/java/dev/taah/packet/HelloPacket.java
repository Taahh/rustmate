package dev.taah.packet;

import dev.taah.connection.PlayerConnection;
import dev.taah.data.PlatformData;
import dev.taah.packet.root.ReactorHandshakePacket;
import dev.taah.util.HazelMessage;
import dev.taah.util.PacketBuffer;

import static java.lang.Math.floor;

/**
 * @author Taah
 * @project crewmate
 * @since 4:59 PM [20-05-2022]
 */
public class HelloPacket extends AbstractPacket<HelloPacket>
{
    private String username;
    private PlatformData platformData;

    private boolean isModded;

    public HelloPacket()
    {
        super(0x08);
    }

    @Override
    public void deserialize(PacketBuffer buffer)
    {
        byte hazelVersion = buffer.readByte();
        int clientVersion = buffer.readInt32();
        this.username = buffer.readString();
        System.out.println("Hazel Version: " + hazelVersion);
        System.out.println("Client Version: " + clientVersion);
        int year = (int) floor(clientVersion / 25000);
        int month = (int) floor((clientVersion %= 25000) / 1800);
        int day = (int) floor((clientVersion %= 1800) / 50);
        int revision = clientVersion % 50;
        System.out.println("Last Nonce Received: " + buffer.readUInt32());
        System.out.println("Last Language: " + buffer.readUInt32());
        System.out.println("Chat Mode Type: " + buffer.readByte());
        HazelMessage platformData = HazelMessage.read(buffer);
        this.platformData = new PlatformData((byte) platformData.getTag(), platformData.getPayload().readString());
        System.out.println("Platform ID and Name: " + this.platformData.platform() + ", " + this.platformData.platformName());
        buffer.readString();
        buffer.readUInt32();

        if (buffer.readableBytes() > 0)
        {
            isModded = true;
            System.out.println("Reactor Protocol Version: " + buffer.readByte());
            System.out.println("Mod Count: " + buffer.readPackedUInt32());
        }

        System.out.println("Received Hello Packet on version " + String.format("%s.%s.%s.%s", year, month, day, revision) + " from user " + username);
    }

    @Override
    public void processPacket(HelloPacket packet, PlayerConnection connection)
    {
        connection.setPlatformData(packet.platformData);
        connection.setClientName(packet.username);
        connection.sendPacket(new AcknowledgePacket(packet.getNonce()));
        if (packet.isModded) {
            System.out.println("SENDING REACTOR HANDSHAKE!");
            connection.sendPacket(new ReactorHandshakePacket());
        }
    }

    @Override
    public int getPacketId()
    {
        return 1;
    }

    @Override
    public void serialize(PacketBuffer buffer)
    {
    }
}
