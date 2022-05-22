package dev.taah.util;
import io.netty.buffer.ByteBuf;
import io.netty.buffer.ByteBufAllocator;
import io.netty.buffer.ByteBufOutputStream;
import io.netty.buffer.Unpooled;
import io.netty.handler.codec.DecoderException;
import io.netty.handler.codec.EncoderException;
import io.netty.util.ByteProcessor;
import org.jetbrains.annotations.Nullable;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.nio.channels.FileChannel;
import java.nio.channels.GatheringByteChannel;
import java.nio.channels.ScatteringByteChannel;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;
import java.util.Collection;
import java.util.Date;
import java.util.UUID;
import java.util.function.BiConsumer;

/**
 * @author Phoenixx
 * RaptureAPI
 * 2020-11-16
 * 11:07 p.m.
 */
public class PacketBuffer extends ByteBuf
{
    private final ByteBuf byteBuf;

    public PacketBuffer()
    {
        this(Unpooled.directBuffer(1));
    }

    public PacketBuffer(ByteBuf wrapped)
    {
        this.byteBuf = wrapped;
    }

    /**
     * Calculates the number of bytes required to fit the supplied int (0-5) if it were to be read/written using
     * readVarIntFromBuffer or writeVarIntToBuffer
     */
    public static int getVarIntSize(int input)
    {
        for (int i = 1; i < 5; ++i) {
            if ((input & -1 << i * 7) == 0) {
                return i;
            }
        }
        return 5;
    }

    public byte[] getByteArray()
    {
        if (byteBuf.hasArray()) {
            return byteBuf.array();
        }
        byte[] bytes = new byte[byteBuf.readableBytes()];
        byteBuf.getBytes(byteBuf.readerIndex(), bytes);
        return bytes;
    }

    public byte[] getByteArraySafe()
    {
        return getByteArraySafe(byteBuf);
    }

    public static byte[] getByteArraySafe(ByteBuf byteBuf)
    {
        if (byteBuf.hasArray()) {
            return byteBuf.array();
        }
        int indexTemp = byteBuf.readerIndex();
        byteBuf.readerIndex(0);
        byte[] bytes = new byte[byteBuf.readableBytes()];
        byteBuf.getBytes(byteBuf.readerIndex(), bytes);
        byteBuf.readerIndex(indexTemp);
        return bytes;
    }

    public PacketBuffer writeByteArray(byte[] array)
    {
        this.writeVarInt(array.length);
        this.writeBytes(array);
        return this;
    }

    public byte[] readByteArray()
    {
        return this.readByteArray(this.readableBytes());
    }

    public byte[] readByteArray(int maxLength)
    {
        int i = this.readVarInt();
        if (i > maxLength) {
            throw new DecoderException("ByteArray with size " + i + " is bigger than allowed " + maxLength);
        } else {
            byte[] abyte = new byte[i];
            this.readBytes(abyte);
            return abyte;
        }
    }

    /**
     * Writes an array of VarInts to the buffer, prefixed by the length of the array (as a VarInt).
     */
    public PacketBuffer writeVarIntArray(int[] array)
    {
        this.writeVarInt(array.length);

        for (int i : array) {
            this.writeVarInt(i);
        }

        return this;
    }

    public int[] readVarIntArray()
    {
        return this.readVarIntArray(this.readableBytes());
    }

    public int[] readVarIntArray(int maxLength)
    {
        int i = this.readVarInt();
        if (i > maxLength) {
            throw new DecoderException("VarIntArray with size " + i + " is bigger than allowed " + maxLength);
        } else {
            int[] aint = new int[i];

            for (int j = 0; j < aint.length; ++j) {
                aint[j] = this.readVarInt();
            }

            return aint;
        }
    }

    /**
     * Writes an array of longs to the buffer, prefixed by the length of the array (as a VarInt).
     */
    public PacketBuffer writeLongArray(long[] array)
    {
        this.writeVarInt(array.length);

        for (long i : array) {
            this.writeLong(i);
        }

        return this;
    }

    /**
     * Reads a length-prefixed array of longs from the buffer.
     */
    public long[] readLongArray(@Nullable long[] array)
    {
        return this.readLongArray(array, this.readableBytes() / 8);
    }

    public long[] readLongArray(@Nullable long[] array, int maxLength)
    {
        int i = this.readVarInt();
        if (array == null || array.length != i) {
            if (i > maxLength) {
                throw new DecoderException("LongArray with size " + i + " is bigger than allowed " + maxLength);
            }

            array = new long[i];
        }

        for (int j = 0; j < array.length; ++j) {
            array[j] = this.readLong();
        }

        return array;
    }

    public <T extends Enum<T>> T readEnumValue(Class<T> enumClass)
    {
        return (T) (enumClass.getEnumConstants())[this.readVarInt()];
    }

    public PacketBuffer writeEnumValue(Enum<?> value)
    {
        return this.writeVarInt(value.ordinal());
    }

    public PacketBuffer writeUUID(UUID pUuid)
    {
        this.writeLong(pUuid.getMostSignificantBits());
        this.writeLong(pUuid.getLeastSignificantBits());
        return this;
    }

    public <T> void writeCollection(Collection<T> pCollection, BiConsumer<PacketBuffer, T> pElementWriter)
    {
        this.writeVarInt(pCollection.size());

        for (T t : pCollection) {
            pElementWriter.accept(this, t);
        }

    }

    /**
     * Reads a UUID encoded as two longs from this buffer.
     *
     * @see #writeUUID
     */
    public UUID readUUID()
    {
        return new UUID(this.readLong(), this.readLong());
    }

    /**
     * Reads a compressed int from the buffer. To do so it maximally reads 5 byte-sized chunks whose most significant bit
     * dictates whether another byte should be read.
     */
    public int readVarInt()
    {
        int i = 0;
        int j = 0;

        while (true) {
            byte b0 = this.readByte();
            i |= (b0 & 127) << j++ * 7;
            if (j > 5) {
                throw new RuntimeException("VarInt too big");
            }

            if ((b0 & 128) != 128) {
                break;
            }
        }

        return i;
    }

    public long readVarLong()
    {
        long i = 0L;
        int j = 0;

        while (true) {
            byte b0 = this.readByte();
            i |= (long) (b0 & 127) << j++ * 7;
            if (j > 10) {
                throw new RuntimeException("VarLong too big");
            }

            if ((b0 & 128) != 128) {
                break;
            }
        }

        return i;
    }

    public PacketBuffer writeUniqueId(UUID uuid)
    {
        this.writeLong(uuid.getMostSignificantBits());
        this.writeLong(uuid.getLeastSignificantBits());
        return this;
    }

    public UUID readUniqueId()
    {
        return new UUID(this.readLong(), this.readLong());
    }

    /**
     * Writes a compressed int to the buffer. The smallest number of bytes to fit the passed int will be written. Of each
     * such byte only 7 bits will be used to describe the actual value since its most significant bit dictates whether
     * the next byte is part of that same int. Micro-optimization for int values that are expected to have values below
     * 128.
     */

    public PacketBuffer writeVarInt(int input)
    {
        while ((input & -128) != 0) {
            this.writeByte(input & 127 | 128);
            input >>>= 7;
        }

        this.writeByte(input);
        return this;
    }

    public PacketBuffer writeVarLong(long value)
    {
        while ((value & -128L) != 0L) {
            this.writeByte((int) (value & 127L) | 128);
            value >>>= 7;
        }

        this.writeByte((int) value);
        return this;
    }

    public String readString()
    {
        return this.readString(32767);
    }

    /**
     * Reads a string from this buffer. Expected parameter is maximum allowed string length. Will throw IOException if
     * string length exceeds this value!
     */
    public String readString(int maxLength)
    {
        int i = this.readVarInt();
        if (i > maxLength * 4) {
            throw new DecoderException("The received encoded string buffer length is longer than maximum allowed (" + i + " > " + maxLength * 4 + ")");
        } else if (i < 0) {
            throw new DecoderException("The received encoded string buffer length is less than zero! Weird string!");
        } else {
            String s = this.toString(this.readerIndex(), i, StandardCharsets.UTF_8);
            this.readerIndex(this.readerIndex() + i);
            if (s.length() > maxLength) {
                throw new DecoderException("The received string length is longer than maximum allowed (" + i + " > " + maxLength + ")");
            } else {
                return s;
            }
        }
    }

    public PacketBuffer writeString(String string)
    {
        return this.writeString(string, 32767);
    }

    public PacketBuffer writeString(String string, int maxLength)
    {
        byte[] abyte = string.getBytes(StandardCharsets.UTF_8);
        if (abyte.length > maxLength) {
            throw new EncoderException("String too big (was " + abyte.length + " bytes encoded, max " + maxLength + ")");
        } else {
            this.writeVarInt(abyte.length);
            this.writeBytes(abyte);
            return this;
        }
    }

    public Date readTime()
    {
        return new Date(this.readLong());
    }

    public PacketBuffer writeTime(Date time)
    {
        this.writeLong(time.getTime());
        return this;
    }

    public int capacity()
    {
        return this.byteBuf.capacity();
    }

    public ByteBuf capacity(int newCapacity)
    {
        return this.byteBuf.capacity(newCapacity);
    }

    public int maxCapacity()
    {
        return this.byteBuf.maxCapacity();
    }

    public ByteBufAllocator alloc()
    {
        return this.byteBuf.alloc();
    }

    public ByteOrder order()
    {
        return this.byteBuf.order();
    }

    public ByteBuf order(ByteOrder byteOrder)
    {
        return this.byteBuf.order(byteOrder);
    }

    public ByteBuf unwrap()
    {
        return this.byteBuf.unwrap();
    }

    public boolean isDirect()
    {
        return this.byteBuf.isDirect();
    }

    public boolean isReadOnly()
    {
        return this.byteBuf.isReadOnly();
    }

    public ByteBuf asReadOnly()
    {
        return this.byteBuf.asReadOnly();
    }

    public int readerIndex()
    {
        return this.byteBuf.readerIndex();
    }

    public ByteBuf readerIndex(int readerIndex)
    {
        return this.byteBuf.readerIndex(readerIndex);
    }

    public int writerIndex()
    {
        return this.byteBuf.writerIndex();
    }

    public ByteBuf writerIndex(int writerIndex)
    {
        return this.byteBuf.writerIndex(writerIndex);
    }

    public ByteBuf setIndex(int readerIndex, int writerIndex)
    {
        return this.byteBuf.setIndex(readerIndex, writerIndex);
    }

    public int readableBytes()
    {
        return this.byteBuf.readableBytes();
    }

    public int writableBytes()
    {
        return this.byteBuf.writableBytes();
    }

    public int maxWritableBytes()
    {
        return this.byteBuf.maxWritableBytes();
    }

    public boolean isReadable()
    {
        return this.byteBuf.isReadable();
    }

    public boolean isReadable(int size)
    {
        return this.byteBuf.isReadable(size);
    }

    public boolean isWritable()
    {
        return this.byteBuf.isWritable();
    }

    public boolean isWritable(int size)
    {
        return this.byteBuf.isWritable(size);
    }

    public ByteBuf clear()
    {
        return this.byteBuf.clear();
    }

    public ByteBuf markReaderIndex()
    {
        return this.byteBuf.markReaderIndex();
    }

    public ByteBuf resetReaderIndex()
    {
        return this.byteBuf.resetReaderIndex();
    }

    public ByteBuf markWriterIndex()
    {
        return this.byteBuf.markWriterIndex();
    }

    public ByteBuf resetWriterIndex()
    {
        return this.byteBuf.resetWriterIndex();
    }

    public ByteBuf discardReadBytes()
    {
        return this.byteBuf.discardReadBytes();
    }

    public ByteBuf discardSomeReadBytes()
    {
        return this.byteBuf.discardSomeReadBytes();
    }

    public ByteBuf ensureWritable(int minWritableBytes)
    {
        return this.byteBuf.ensureWritable(minWritableBytes);
    }

    public int ensureWritable(int minWritableBytes, boolean force)
    {
        return this.byteBuf.ensureWritable(minWritableBytes, force);
    }

    public boolean getBoolean(int index)
    {
        return this.byteBuf.getBoolean(index);
    }

    public byte getByte(int index)
    {
        return this.byteBuf.getByte(index);
    }

    public short getUnsignedByte(int index)
    {
        return this.byteBuf.getUnsignedByte(index);
    }

    public short getShort(int index)
    {
        return this.byteBuf.getShort(index);
    }

    public short getShortLE(int index)
    {
        return this.byteBuf.getShortLE(index);
    }

    public int getUnsignedShort(int index)
    {
        return this.byteBuf.getUnsignedShort(index);
    }

    public int getUnsignedShortLE(int index)
    {
        return this.byteBuf.getUnsignedShortLE(index);
    }

    public int getMedium(int index)
    {
        return this.byteBuf.getMedium(index);
    }

    public int getMediumLE(int index)
    {
        return this.byteBuf.getMediumLE(index);
    }

    public int getUnsignedMedium(int index)
    {
        return this.byteBuf.getUnsignedMedium(index);
    }

    public int getUnsignedMediumLE(int index)
    {
        return this.byteBuf.getUnsignedMediumLE(index);
    }

    public int getInt(int index)
    {
        return this.byteBuf.getInt(index);
    }

    public int getIntLE(int index)
    {
        return this.byteBuf.getIntLE(index);
    }

    public long getUnsignedInt(int index)
    {
        return this.byteBuf.getUnsignedInt(index);
    }

    public long getUnsignedIntLE(int index)
    {
        return this.byteBuf.getUnsignedIntLE(index);
    }

    public long getLong(int index)
    {
        return this.byteBuf.getLong(index);
    }

    public long getLongLE(int index)
    {
        return this.byteBuf.getLongLE(index);
    }

    public char getChar(int index)
    {
        return this.byteBuf.getChar(index);
    }

    public float getFloat(int index)
    {
        return this.byteBuf.getFloat(index);
    }

    public double getDouble(int index)
    {
        return this.byteBuf.getDouble(index);
    }

    public ByteBuf getBytes(int index, ByteBuf dst)
    {
        return this.byteBuf.getBytes(index, dst);
    }

    public ByteBuf getBytes(int index, ByteBuf dst, int length)
    {
        return this.byteBuf.getBytes(index, dst, length);
    }

    public ByteBuf getBytes(int index, ByteBuf dst, int dstIndex, int length)
    {
        return this.byteBuf.getBytes(index, dst, dstIndex, length);
    }

    public ByteBuf getBytes(int index, byte[] dst)
    {
        return this.byteBuf.getBytes(index, dst);
    }

    public ByteBuf getBytes(int index, byte[] dst, int dstIndex, int length)
    {
        return this.byteBuf.getBytes(index, dst, dstIndex, length);
    }

    public ByteBuf getBytes(int index, ByteBuffer dst)
    {
        return this.byteBuf.getBytes(index, dst);
    }

    public ByteBuf getBytes(int index, OutputStream out, int length) throws IOException
    {
        return this.byteBuf.getBytes(index, out, length);
    }

    public int getBytes(int index, GatheringByteChannel out, int length) throws IOException
    {
        return this.byteBuf.getBytes(index, out, length);
    }

    public int getBytes(int index, FileChannel out, long position, int length) throws IOException
    {
        return this.byteBuf.getBytes(index, out, position, length);
    }

    public CharSequence getCharSequence(int index, int length, Charset charset)
    {
        return this.byteBuf.getCharSequence(index, length, charset);
    }

    public ByteBuf setBoolean(int index, boolean value)
    {
        return this.byteBuf.setBoolean(index, value);
    }

    public ByteBuf setByte(int index, int value)
    {
        return this.byteBuf.setByte(index, value);
    }

    public ByteBuf setShort(int index, int value)
    {
        return this.byteBuf.setShort(index, value);
    }

    public ByteBuf setShortLE(int index, int value)
    {
        return this.byteBuf.setShortLE(index, value);
    }

    public ByteBuf setMedium(int index, int value)
    {
        return this.byteBuf.setMedium(index, value);
    }

    public ByteBuf setMediumLE(int index, int value)
    {
        return this.byteBuf.setMediumLE(index, value);
    }

    public ByteBuf setInt(int index, int value)
    {
        return this.byteBuf.setInt(index, value);
    }

    public ByteBuf setIntLE(int index, int value)
    {
        return this.byteBuf.setIntLE(index, value);
    }

    public ByteBuf setLong(int index, long value)
    {
        return this.byteBuf.setLong(index, value);
    }

    public ByteBuf setLongLE(int index, long value)
    {
        return this.byteBuf.setLongLE(index, value);
    }

    public ByteBuf setChar(int index, int value)
    {
        return this.byteBuf.setChar(index, value);
    }

    public ByteBuf setFloat(int index, float value)
    {
        return this.byteBuf.setFloat(index, value);
    }

    public ByteBuf setDouble(int index, double value)
    {
        return this.byteBuf.setDouble(index, value);
    }

    public ByteBuf setBytes(int index, ByteBuf src)
    {
        return this.byteBuf.setBytes(index, src);
    }

    public ByteBuf setBytes(int index, ByteBuf src, int length)
    {
        return this.byteBuf.setBytes(index, src, length);
    }

    public ByteBuf setBytes(int index, ByteBuf src, int srcIndex, int length)
    {
        return this.byteBuf.setBytes(index, src, srcIndex, length);
    }

    public ByteBuf setBytes(int index, byte[] src)
    {
        return this.byteBuf.setBytes(index, src);
    }

    public ByteBuf setBytes(int index, byte[] src, int srcIndex, int length)
    {
        return this.byteBuf.setBytes(index, src, srcIndex, length);
    }

    public ByteBuf setBytes(int index, ByteBuffer src)
    {
        return this.byteBuf.setBytes(index, src);
    }

    public int setBytes(int index, InputStream inputStream, int length) throws IOException
    {
        return this.byteBuf.setBytes(index, inputStream, length);
    }

    public int setBytes(int index, ScatteringByteChannel in, int length) throws IOException
    {
        return this.byteBuf.setBytes(index, in, length);
    }

    public int setBytes(int index, FileChannel in, long position, int length) throws IOException
    {
        return this.byteBuf.setBytes(index, in, position, length);
    }

    public ByteBuf setZero(int index, int length)
    {
        return this.byteBuf.setZero(index, length);
    }

    public int setCharSequence(int index, CharSequence charSequence, Charset charset)
    {
        return this.byteBuf.setCharSequence(index, charSequence, charset);
    }

    public short readInt16()
    {
        return this.readShortLE();
    }

    public int readInt32()
    {
        return this.readIntLE();
    }

    public int readUInt16()
    {
        return this.readUnsignedShortLE();
    }

    public long readUInt32()
    {
        return this.readUnsignedIntLE();
    }

    public String readPackedString()
    {
        int len = (int) this.readPackedUInt32();
        CharSequence charSequence = byteBuf.readCharSequence(len, Charset.defaultCharset());
        return charSequence.toString();
    }

    public long readPackedUInt32()
    {
        boolean readMore = true;
        int shift = 0;
        long output = 0;

        while (readMore) {

            long b = (long) byteBuf.readUnsignedByte();
            if (b >= 0x80) {
                readMore = true;
                b ^= 0x80;
            } else {
                readMore = false;
            }
            output |= (long) (b << shift);
            shift += 7;
        }

        return output;
    }

    public boolean readBoolean()
    {
        return this.byteBuf.readBoolean();
    }

    public byte readByte()
    {
        return this.byteBuf.readByte();
    }

    public short readUnsignedByte()
    {
        return this.byteBuf.readUnsignedByte();
    }

    public short readShort()
    {
        return this.byteBuf.readShort();
    }

    public short readShortLE()
    {
        return this.byteBuf.readShortLE();
    }

    public int readUnsignedShort()
    {
        return this.byteBuf.readUnsignedShort();
    }

    public int readUnsignedShortLE()
    {
        return this.byteBuf.readUnsignedShortLE();
    }

    public int readMedium()
    {
        return this.byteBuf.readMedium();
    }

    public int readMediumLE()
    {
        return this.byteBuf.readMediumLE();
    }

    public int readUnsignedMedium()
    {
        return this.byteBuf.readUnsignedMedium();
    }

    public int readUnsignedMediumLE()
    {
        return this.byteBuf.readUnsignedMediumLE();
    }

    public int readInt()
    {
        return this.byteBuf.readInt();
    }

    public int readIntLE()
    {
        return this.byteBuf.readIntLE();
    }

    public long readUnsignedInt()
    {
        return this.byteBuf.readUnsignedInt();
    }

    public long readUnsignedIntLE()
    {
        return this.byteBuf.readUnsignedIntLE();
    }

    public long readLong()
    {
        return this.byteBuf.readLong();
    }

    public long readLongLE()
    {
        return this.byteBuf.readLongLE();
    }

    public char readChar()
    {
        return this.byteBuf.readChar();
    }

    public float readFloat()
    {
        return this.byteBuf.readFloat();
    }

    public double readDouble()
    {
        return this.byteBuf.readDouble();
    }

    public ByteBuf readBytes(int length)
    {
        return this.byteBuf.readBytes(length);
    }

    public ByteBuf readSlice(int length)
    {
        return this.byteBuf.readSlice(length);
    }

    public ByteBuf readRetainedSlice(int length)
    {
        return this.byteBuf.readRetainedSlice(length);
    }

    public ByteBuf readBytes(ByteBuf dst)
    {
        return this.byteBuf.readBytes(dst);
    }

    public ByteBuf readBytes(ByteBuf dst, int length)
    {
        return this.byteBuf.readBytes(dst, length);
    }

    public ByteBuf readBytes(ByteBuf dst, int dstIndex, int length)
    {
        return this.byteBuf.readBytes(dst, dstIndex, length);
    }

    public ByteBuf readBytes(byte[] dst)
    {
        return this.byteBuf.readBytes(dst);
    }

    public ByteBuf readBytes(byte[] dst, int dstIndex, int length)
    {
        return this.byteBuf.readBytes(dst, dstIndex, length);
    }

    public ByteBuf readBytes(ByteBuffer dst)
    {
        return this.byteBuf.readBytes(dst);
    }

    public ByteBuf readBytes(OutputStream out, int length) throws IOException
    {
        return this.byteBuf.readBytes(out, length);
    }

    public int readBytes(GatheringByteChannel out, int length) throws IOException
    {
        return this.byteBuf.readBytes(out, length);
    }

    public CharSequence readCharSequence(int length, Charset charset)
    {
        return this.byteBuf.readCharSequence(length, charset);
    }

    public int readBytes(FileChannel out, long position, int length) throws IOException
    {
        return this.byteBuf.readBytes(out, position, length);
    }

    public ByteBuf skipBytes(int position)
    {
        return this.byteBuf.skipBytes(position);
    }

    public ByteBuf writeBoolean(boolean value)
    {
        return this.byteBuf.writeBoolean(value);
    }

    public void writeInt16(short value)
    {
        this.writeShortLE(value);
    }

    public void writeInt32(int value)
    {
        this.writeIntLE(value);
    }

    public void writePackedInt32(int value)
    {
        this.writePackedUInt32(Integer.toUnsignedLong(value));
    }

    public void writeUInt16(int value)
    {
        this.writeShortLE(value);
    }

    public void writeUInt32(long value)
    {
        this.writeLongLE(value);
    }

    public void writePackedUInt32(long value)
    {
        do {
            long b = value;
            if (value >= (0x80 & 0xFF)) {
                b |= (0x80 & 0xFF);
            }
            this.writeByte((short) b);
            value >>= 7;
        } while (value > 0);
    }

    public ByteBuf writeByte(int value)
    {
        return this.byteBuf.writeByte(value);
    }

    public ByteBuf writeShort(int value)
    {
        return this.byteBuf.writeShort(value);
    }

    public ByteBuf writeShortLE(int value)
    {
        return this.byteBuf.writeShortLE(value);
    }

    public ByteBuf writeMedium(int value)
    {
        return this.byteBuf.writeMedium(value);
    }

    public ByteBuf writeMediumLE(int value)
    {
        return this.byteBuf.writeMediumLE(value);
    }

    public ByteBuf writeInt(int value)
    {
        return this.byteBuf.writeInt(value);
    }

    public ByteBuf writeIntLE(int value)
    {
        return this.byteBuf.writeIntLE(value);
    }

    public ByteBuf writeLong(long value)
    {
        return this.byteBuf.writeLong(value);
    }

    public ByteBuf writeLongLE(long value)
    {
        return this.byteBuf.writeLongLE(value);
    }

    public ByteBuf writeChar(int value)
    {
        return this.byteBuf.writeChar(value);
    }

    public ByteBuf writeFloat(float value)
    {
        return this.byteBuf.writeFloat(value);
    }

    public ByteBuf writeDouble(double value)
    {
        return this.byteBuf.writeDouble(value);
    }

    public ByteBuf writeBytes(ByteBuf src)
    {
        return this.byteBuf.writeBytes(src);
    }

    public ByteBuf writeBytes(ByteBuf src, int length)
    {
        return this.byteBuf.writeBytes(src, length);
    }

    public ByteBuf writeBytes(ByteBuf src, int srcIndex, int length)
    {
        return this.byteBuf.writeBytes(src, srcIndex, length);
    }

    public ByteBuf writeBytes(byte[] src)
    {
        return this.byteBuf.writeBytes(src);
    }

    public ByteBuf writeBytes(byte[] src, int srcIndex, int length)
    {
        return this.byteBuf.writeBytes(src, srcIndex, length);
    }

    public ByteBuf writeBytes(ByteBuffer src)
    {
        return this.byteBuf.writeBytes(src);
    }

    public int writeBytes(InputStream in, int length) throws IOException
    {
        return this.byteBuf.writeBytes(in, length);
    }

    public int writeBytes(ScatteringByteChannel in, int length) throws IOException
    {
        return this.byteBuf.writeBytes(in, length);
    }

    public int writeBytes(FileChannel in, long position, int length) throws IOException
    {
        return this.byteBuf.writeBytes(in, position, length);
    }

    public ByteBuf writeZero(int length)
    {
        return this.byteBuf.writeZero(length);
    }

    public int writeCharSequence(CharSequence charSequence, Charset charset)
    {
        return this.byteBuf.writeCharSequence(charSequence, charset);
    }

    public int indexOf(int fromIndex, int toIndex, byte value)
    {
        return this.byteBuf.indexOf(fromIndex, toIndex, value);
    }

    public int bytesBefore(byte value)
    {
        return this.byteBuf.bytesBefore(value);
    }

    public int bytesBefore(int length, byte value)
    {
        return this.byteBuf.bytesBefore(length, value);
    }

    public int bytesBefore(int index, int length, byte value)
    {
        return this.byteBuf.bytesBefore(index, length, value);
    }

    public int forEachByte(ByteProcessor processor)
    {
        return this.byteBuf.forEachByte(processor);
    }

    public int forEachByte(int index, int length, ByteProcessor processor)
    {
        return this.byteBuf.forEachByte(index, length, processor);
    }

    public int forEachByteDesc(ByteProcessor processor)
    {
        return this.byteBuf.forEachByteDesc(processor);
    }

    public int forEachByteDesc(int index, int length, ByteProcessor processor)
    {
        return this.byteBuf.forEachByteDesc(index, length, processor);
    }

    public ByteBuf copy()
    {
        return this.byteBuf.copy();
    }

    public PacketBuffer copyPacketBuffer()
    {
        return new PacketBuffer(this.byteBuf.copy());
    }

    public ByteBuf copy(int index, int length)
    {
        return this.byteBuf.copy(index, length);
    }

    public ByteBuf slice()
    {
        return this.byteBuf.slice();
    }

    public ByteBuf retainedSlice()
    {
        return this.byteBuf.retainedSlice();
    }

    public ByteBuf slice(int index, int length)
    {
        return this.byteBuf.slice(index, length);
    }

    public ByteBuf retainedSlice(int index, int length)
    {
        return this.byteBuf.retainedSlice(index, length);
    }

    public ByteBuf duplicate()
    {
        return this.byteBuf.duplicate();
    }

    public ByteBuf retainedDuplicate()
    {
        return this.byteBuf.retainedDuplicate();
    }

    public int nioBufferCount()
    {
        return this.byteBuf.nioBufferCount();
    }

    public ByteBuffer nioBuffer()
    {
        return this.byteBuf.nioBuffer();
    }

    public ByteBuffer nioBuffer(int index, int length)
    {
        return this.byteBuf.nioBuffer(index, length);
    }

    public ByteBuffer internalNioBuffer(int index, int length)
    {
        return this.byteBuf.internalNioBuffer(index, length);
    }

    public ByteBuffer[] nioBuffers()
    {
        return this.byteBuf.nioBuffers();
    }

    public ByteBuffer[] nioBuffers(int index, int length)
    {
        return this.byteBuf.nioBuffers(index, length);
    }

    public boolean hasArray()
    {
        return this.byteBuf.hasArray();
    }

    public byte[] array()
    {
        return this.byteBuf.array();
    }

    public int arrayOffset()
    {
        return this.byteBuf.arrayOffset();
    }

    public boolean hasMemoryAddress()
    {
        return this.byteBuf.hasMemoryAddress();
    }

    public long memoryAddress()
    {
        return this.byteBuf.memoryAddress();
    }

    public String toString(Charset charset)
    {
        return this.byteBuf.toString(charset);
    }

    public String toString(int index, int length, Charset charset)
    {
        return this.byteBuf.toString(index, length, charset);
    }

    public int hashCode()
    {
        return this.byteBuf.hashCode();
    }

    public boolean equals(Object obj)
    {
        return this.byteBuf.equals(obj);
    }

    public int compareTo(ByteBuf buffer)
    {
        return this.byteBuf.compareTo(buffer);
    }

    public String toString()
    {
        return this.byteBuf.toString();
    }

    public ByteBuf retain(int increment)
    {
        return this.byteBuf.retain(increment);
    }

    public ByteBuf retain()
    {
        return this.byteBuf.retain();
    }

    public ByteBuf touch()
    {
        return this.byteBuf.touch();
    }

    public ByteBuf touch(Object hint)
    {
        return this.byteBuf.touch(hint);
    }

    public int refCnt()
    {
        return this.byteBuf.refCnt();
    }

    public boolean release()
    {
        return this.byteBuf.release();
    }

    public boolean release(int decrement)
    {
        return this.byteBuf.release(decrement);
    }

    public ByteBuf getByteBuf()
    {
        return byteBuf;
    }
}
