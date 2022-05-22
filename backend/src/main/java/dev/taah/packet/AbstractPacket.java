package dev.taah.packet;

import dev.taah.connection.PlayerConnection;
import dev.taah.util.PacketBuffer;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.With;

/**
 * @author Taah
 * @project crewmate
 * @since 5:44 PM [20-05-2022]
 */
@Data
public abstract class AbstractPacket<T extends AbstractPacket<?>> implements ISerializable, IDeserializable
{
    private final int packetId;
    private int nonce;
    public AbstractPacket(int packetId) {
        this(packetId, -1);
    }

    public AbstractPacket(int packetId, int nonce) {
        this.packetId = packetId;
        this.nonce = nonce;
    }

    public abstract void processPacket(T packet, PlayerConnection connection);


    public int getNonce()
    {
        return nonce;
    }
}
